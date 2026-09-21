// Browser-side helpers invoked from Blazor components.
//
// The starfield below is a direct port of the animation that the Rust build
// generated and injected as a <script> tag at runtime.

window.site = (function () {
    // Guards against a second animation loop if the component ever remounts.
    const initialized = new Set();

    function initStarfield(canvasId, config) {
        if (initialized.has(canvasId)) return;

        const canvas = document.getElementById(canvasId);
        if (!canvas) return;

        initialized.add(canvasId);

        const starOpacity = config.starOpacity;
        const speedMultiplier = config.speedMultiplier;

        const ctx = canvas.getContext('2d');
        let width = window.innerWidth;
        let height = window.innerHeight;

        canvas.width = width;
        canvas.height = height;

        class Star {
            constructor() {
                this.x = Math.random() * width;
                this.y = Math.random() * height;
                this.size = Math.random() * 2 + 0.5;
                this.opacity = Math.random() * starOpacity;
                this.twinkleSpeed = Math.random() * 0.02 + 0.005;
                this.twinkleDirection = Math.random() > 0.5 ? 1 : -1;
            }

            draw() {
                ctx.fillStyle = `rgba(255, 255, 255, ${this.opacity})`;
                ctx.beginPath();
                ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
                ctx.fill();
            }

            update() {
                this.opacity += this.twinkleSpeed * this.twinkleDirection;
                if (this.opacity >= starOpacity || this.opacity <= 0.1) {
                    this.twinkleDirection *= -1;
                }
            }
        }

        class ShootingStar {
            constructor() {
                this.reset();
            }

            reset() {
                this.x = Math.random() * width;
                this.y = Math.random() * height * 0.5;
                this.length = Math.random() * 80 + 40;
                this.speed = (Math.random() * 8 + 4) * speedMultiplier;
                this.angle = Math.PI / 4;
                this.opacity = 0;
                this.fadeIn = true;
                this.life = 0;
                this.maxLife = Math.random() * 100 + 100;
                this.color = {
                    r: Math.floor(Math.random() * 50 + 205),
                    g: Math.floor(Math.random() * 50 + 205),
                    b: 255
                };
            }

            draw() {
                ctx.save();
                const gradient = ctx.createLinearGradient(
                    this.x, this.y,
                    this.x - Math.cos(this.angle) * this.length,
                    this.y - Math.sin(this.angle) * this.length
                );
                gradient.addColorStop(0, `rgba(${this.color.r}, ${this.color.g}, ${this.color.b}, ${this.opacity})`);
                gradient.addColorStop(1, `rgba(${this.color.r}, ${this.color.g}, ${this.color.b}, 0)`);
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
            }

            update() {
                this.life++;
                if (this.fadeIn && this.opacity < 1) {
                    this.opacity += 0.05;
                    if (this.opacity >= 1) this.fadeIn = false;
                }
                if (this.life > this.maxLife * 0.7) {
                    this.opacity -= 0.02;
                }
                this.x += Math.cos(this.angle) * this.speed;
                this.y += Math.sin(this.angle) * this.speed;
                if (this.x > width + this.length ||
                    this.y > height + this.length ||
                    this.opacity <= 0 ||
                    this.life > this.maxLife) {
                    this.reset();
                }
            }
        }

        const stars = [];
        for (let i = 0; i < config.starCount; i++) {
            stars.push(new Star());
        }

        const shootingStars = [];
        for (let i = 0; i < config.shootingStarCount; i++) {
            shootingStars.push(new ShootingStar());
        }

        function animate() {
            ctx.clearRect(0, 0, width, height);
            stars.forEach(star => {
                star.update();
                star.draw();
            });
            shootingStars.forEach(s => {
                s.update();
                s.draw();
            });
            requestAnimationFrame(animate);
        }

        window.addEventListener('resize', () => {
            width = window.innerWidth;
            height = window.innerHeight;
            canvas.width = width;
            canvas.height = height;
            stars.forEach(star => {
                if (star.x > width) star.x = Math.random() * width;
                if (star.y > height) star.y = Math.random() * height;
            });
        });

        animate();
    }

    // Blazor sets `muted` as an attribute after the element is created, but Chrome's
    // autoplay policy reads the DOM property. Set it directly, then start playback.
    function primeVideo(video) {
        if (!video) return;
        video.muted = true;
        video.defaultMuted = true;
        const played = video.play();
        if (played && typeof played.catch === 'function') {
            played.catch(() => { /* autoplay refused; the poster frame stays */ });
        }
    }

    return { initStarfield, primeVideo };
})();
