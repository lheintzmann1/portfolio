//! # Starfield Background Component
//!
//! A canvas-based starfield with animated shooting stars that covers the entire page.

use dioxus::prelude::*;

/// Starfield configuration options
#[derive(Clone, Copy, PartialEq)]
pub struct StarfieldConfig {
    /// Number of static stars
    pub star_count: usize,
    /// Number of shooting stars
    pub shooting_star_count: usize,
    /// Star opacity (0.0 - 1.0)
    pub star_opacity: f32,
    /// Shooting star speed multiplier
    pub speed_multiplier: f32,
}

impl Default for StarfieldConfig {
    fn default() -> Self {
        Self {
            star_count: 360,
            shooting_star_count: 3,
            star_opacity: 0.6,
            speed_multiplier: 1.0,
        }
    }
}

/// Starfield background with shooting stars effect.
#[component]
pub fn Starfield(#[props(default)] config: StarfieldConfig) -> Element {
    let canvas_id = "starfield-canvas";

    rsx! {
        canvas {
            id: "{canvas_id}",
            class: "fixed top-0 left-0 w-full pointer-events-none",
            style: "z-index: 1; height: 100vh;",
            onmounted: move |_| {
                initialize_starfield(canvas_id, config);
            },
        }
    }
}

/// Initializes the starfield canvas with JavaScript
fn initialize_starfield(canvas_id: &str, config: StarfieldConfig) {
    let script = format!(
        r#"
        (function() {{
            const canvas = document.getElementById('{canvas_id}');
            if (!canvas) return;
            
            const ctx = canvas.getContext('2d');
            let width = window.innerWidth;
            let height = window.innerHeight;
            
            canvas.width = width;
            canvas.height = height;
            
            class Star {{
                constructor() {{
                    this.x = Math.random() * width;
                    this.y = Math.random() * height;
                    this.size = Math.random() * 2 + 0.5;
                    this.opacity = Math.random() * {star_opacity};
                    this.twinkleSpeed = Math.random() * 0.02 + 0.005;
                    this.twinkleDirection = Math.random() > 0.5 ? 1 : -1;
                }}
                
                draw() {{
                    ctx.fillStyle = `rgba(255, 255, 255, ${{this.opacity}})`;
                    ctx.beginPath();
                    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
                    ctx.fill();
                }}
                
                update() {{
                    this.opacity += this.twinkleSpeed * this.twinkleDirection;
                    if (this.opacity >= {star_opacity} || this.opacity <= 0.1) {{
                        this.twinkleDirection *= -1;
                    }}
                }}
            }}
            
            class ShootingStar {{
                constructor() {{
                    this.reset();
                }}
                
                reset() {{
                    this.x = Math.random() * width;
                    this.y = Math.random() * height * 0.5;
                    this.length = Math.random() * 80 + 40;
                    this.speed = (Math.random() * 8 + 4) * {speed_multiplier};
                    this.angle = Math.PI / 4;
                    this.opacity = 0;
                    this.fadeIn = true;
                    this.life = 0;
                    this.maxLife = Math.random() * 100 + 100;
                    this.color = {{
                        r: Math.floor(Math.random() * 50 + 205),
                        g: Math.floor(Math.random() * 50 + 205),
                        b: 255
                    }};
                }}
                
                draw() {{
                    ctx.save();
                    const gradient = ctx.createLinearGradient(
                        this.x, this.y,
                        this.x - Math.cos(this.angle) * this.length,
                        this.y - Math.sin(this.angle) * this.length
                    );
                    gradient.addColorStop(0, `rgba(${{this.color.r}}, ${{this.color.g}}, ${{this.color.b}}, ${{this.opacity}})`);
                    gradient.addColorStop(1, `rgba(${{this.color.r}}, ${{this.color.g}}, ${{this.color.b}}, 0)`);
                    ctx.strokeStyle = gradient;
                    ctx.lineWidth = 2;
                    ctx.lineCap = 'round';
                    ctx.beginPath();
                    ctx.moveTo(this.x, this.y);
                    ctx.lineTo(
                        this.x - Math.cos(this.angle) * this.length,
                        this.y - Math.sin(this.angle) * this.length
                    );
                    ctx.stroke();
                    ctx.restore();
                }}
                
                update() {{
                    this.life++;
                    if (this.fadeIn && this.opacity < 1) {{
                        this.opacity += 0.05;
                        if (this.opacity >= 1) this.fadeIn = false;
                    }}
                    if (this.life > this.maxLife * 0.7) {{
                        this.opacity -= 0.02;
                    }}
                    this.x += Math.cos(this.angle) * this.speed;
                    this.y += Math.sin(this.angle) * this.speed;
                    if (this.x > width + this.length || 
                        this.y > height + this.length || 
                        this.opacity <= 0 || 
                        this.life > this.maxLife) {{
                        this.reset();
                    }}
                }}
            }}
            
            const stars = [];
            for (let i = 0; i < {star_count}; i++) {{
                stars.push(new Star());
            }}
            
            const shootingStars = [];
            for (let i = 0; i < {shooting_star_count}; i++) {{
                shootingStars.push(new ShootingStar());
            }}
            
            function animate() {{
                ctx.clearRect(0, 0, width, height);
                stars.forEach(star => {{
                    star.update();
                    star.draw();
                }});
                shootingStars.forEach(s => {{
                    s.update();
                    s.draw();
                }});
                requestAnimationFrame(animate);
            }}
            
            window.addEventListener('resize', () => {{
                width = window.innerWidth;
                height = window.innerHeight;
                canvas.width = width;
                canvas.height = height;
                stars.forEach(star => {{
                    if (star.x > width) star.x = Math.random() * width;
                    if (star.y > height) star.y = Math.random() * height;
                }});
            }});
            
            animate();
        }})();
    "#,
        canvas_id = canvas_id,
        star_count = config.star_count,
        shooting_star_count = config.shooting_star_count,
        star_opacity = config.star_opacity,
        speed_multiplier = config.speed_multiplier,
    );

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(script_el) = document.create_element("script") {
                    script_el.set_text_content(Some(&script));
                    if let Some(body) = document.body() {
                        let _ = body.append_child(&script_el);
                    }
                }
            }
        }
    }
}
