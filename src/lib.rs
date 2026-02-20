pub mod app;
pub mod components;
pub mod pages;
pub mod posts;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
    setup_particles();
}

#[cfg(feature = "hydrate")]
fn setup_particles() {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };

    let script = document.create_element("script").ok();
    if let Some(script) = script {
        script.set_text_content(Some(PARTICLE_JS));
        let _ = document.body().map(|b| b.append_child(&script));
    }
}

#[cfg(feature = "hydrate")]
const PARTICLE_JS: &str = r#"
(function() {
    const canvas = document.createElement('canvas');
    canvas.id = 'particle-canvas';
    document.body.insertBefore(canvas, document.body.firstChild);

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const dpr = window.devicePixelRatio || 1;
    let W, H;

    function resize() {
        W = window.innerWidth;
        H = window.innerHeight;
        canvas.width = W * dpr;
        canvas.height = H * dpr;
        canvas.style.width = W + 'px';
        canvas.style.height = H + 'px';
    }
    resize();
    window.addEventListener('resize', resize);

    function rand(min, max) { return Math.random() * (max - min) + min; }

    // Everforest palette
    // green: #A7C080 → 167,192,128
    // gold:  #DBBC7F → 219,188,127
    // aqua:  #83C092 → 131,192,146
    // rose:  #D699B6 → 214,153,182
    // blue:  #7FBBB3 → 127,187,179
    // fg:    #D3C6AA → 211,198,170

    // Firefly colors — mix of green and gold, like real fireflies in a garden
    const fireflyColors = [
        [167, 192, 128],  // Everforest green
        [219, 188, 127],  // Everforest gold
        [131, 192, 146],  // Everforest aqua
        [219, 188, 127],  // gold again (more common)
    ];

    const FIREFLY_COUNT = 20;
    const DUST_COUNT = 50;
    const PETAL_COUNT = 6;

    // Fireflies — soft glowing orbs
    const fireflies = [];
    for (let i = 0; i < FIREFLY_COUNT; i++) {
        const color = fireflyColors[Math.floor(rand(0, fireflyColors.length))];
        fireflies.push({
            x: rand(0, W),
            y: rand(0, H),
            vx: rand(-0.12, 0.12),
            vy: rand(-0.08, 0.08),
            size: rand(1.2, 2.5),
            phase: rand(0, Math.PI * 2),
            pulseSpeed: rand(0.2, 0.6),
            brightness: rand(0.25, 0.75),
            r: color[0], g: color[1], b: color[2],
        });
    }

    // Dust motes — Everforest fg color, barely visible
    const dust = [];
    for (let i = 0; i < DUST_COUNT; i++) {
        dust.push({
            x: rand(0, W),
            y: rand(0, H),
            vx: rand(-0.04, 0.04),
            vy: rand(-0.02, 0.02),
            size: rand(0.4, 1.2),
            alpha: rand(0.03, 0.12),
            phase: rand(0, Math.PI * 2),
        });
    }

    // Petals — Everforest rose, green, aqua
    const petalColors = [
        [214, 153, 182],  // Everforest rose/purple
        [167, 192, 128],  // Everforest green
        [131, 192, 146],  // Everforest aqua
        [127, 187, 179],  // Everforest blue
    ];

    const petals = [];
    for (let i = 0; i < PETAL_COUNT; i++) {
        petals.push({
            x: rand(0, W),
            y: rand(-H * 0.3, H),
            vx: rand(-0.06, 0.1),
            vy: rand(0.03, 0.15),
            rotation: rand(0, Math.PI * 2),
            rotSpeed: rand(-0.004, 0.004),
            size: rand(2, 4),
            alpha: rand(0.04, 0.1),
            hue: Math.floor(rand(0, petalColors.length)),
        });
    }

    let time = 0;
    const BREATH_PERIOD = 8.0;

    function animate() {
        time += 1/60;
        const breath = Math.sin(time * Math.PI * 2 / BREATH_PERIOD);
        const breathFactor = 0.5 + breath * 0.5;

        ctx.save();
        ctx.scale(dpr, dpr);
        ctx.clearRect(0, 0, W, H);

        // Dust — warm fg-colored specks
        for (const d of dust) {
            d.x += d.vx + breath * 0.015;
            d.y += d.vy + Math.sin(time * 0.7 + d.phase) * 0.008;
            if (d.x < -10) d.x = W + 10;
            if (d.x > W + 10) d.x = -10;
            if (d.y < -10) d.y = H + 10;
            if (d.y > H + 10) d.y = -10;

            const a = d.alpha * (0.4 + breathFactor * 0.6);
            ctx.beginPath();
            ctx.arc(d.x, d.y, d.size, 0, Math.PI * 2);
            ctx.fillStyle = 'rgba(211, 198, 170, ' + a + ')';
            ctx.fill();
        }

        // Fireflies — color-varied, soft glow
        for (const f of fireflies) {
            f.phase += f.pulseSpeed / 60;
            const pulse = Math.sin(f.phase);
            const glow = f.brightness * (0.3 + pulse * 0.35 + breathFactor * 0.35);

            f.x += f.vx + breath * 0.06;
            f.y += f.vy + Math.sin(time * 0.4 + f.phase) * 0.12;

            f.vx += rand(-0.002, 0.002);
            f.vy += rand(-0.002, 0.002);
            f.vx *= 0.998;
            f.vy *= 0.998;

            if (f.x < -20) f.x = W + 20;
            if (f.x > W + 20) f.x = -20;
            if (f.y < -20) f.y = H + 20;
            if (f.y > H + 20) f.y = -20;

            // Outer glow — same color as core, just diffused
            const grad = ctx.createRadialGradient(f.x, f.y, 0, f.x, f.y, f.size * 10);
            grad.addColorStop(0, 'rgba(' + f.r + ',' + f.g + ',' + f.b + ',' + (glow * 0.12) + ')');
            grad.addColorStop(0.3, 'rgba(' + f.r + ',' + f.g + ',' + f.b + ',' + (glow * 0.04) + ')');
            grad.addColorStop(1, 'rgba(' + f.r + ',' + f.g + ',' + f.b + ',0)');
            ctx.beginPath();
            ctx.arc(f.x, f.y, f.size * 10, 0, Math.PI * 2);
            ctx.fillStyle = grad;
            ctx.fill();

            // Core — bright center
            ctx.beginPath();
            ctx.arc(f.x, f.y, f.size, 0, Math.PI * 2);
            ctx.fillStyle = 'rgba(' + f.r + ',' + f.g + ',' + f.b + ',' + glow + ')';
            ctx.fill();
        }

        // Petals — gentle fall
        for (const p of petals) {
            p.x += p.vx + breath * 0.04;
            p.y += p.vy;
            p.rotation += p.rotSpeed;

            if (p.y > H + 20) { p.y = -20; p.x = rand(0, W); }
            if (p.x < -20) p.x = W + 20;
            if (p.x > W + 20) p.x = -20;

            const c = petalColors[p.hue];
            const a = p.alpha * (0.5 + breathFactor * 0.5);

            ctx.save();
            ctx.translate(p.x, p.y);
            ctx.rotate(p.rotation);
            ctx.beginPath();
            ctx.ellipse(0, 0, p.size * 0.35, p.size, 0, 0, Math.PI * 2);
            ctx.fillStyle = 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a + ')';
            ctx.fill();
            ctx.restore();
        }

        ctx.restore();
        requestAnimationFrame(animate);
    }

    requestAnimationFrame(animate);
})();
"#;
