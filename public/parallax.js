window.addEventListener('scroll', function() {
    var y = window.scrollY * 0.3;
    var blur = document.getElementById('bg-blur');
    var overlay = document.getElementById('bg-overlay');
    if (blur) blur.style.transform = 'translate3d(0,' + y + 'px,0)';
    if (overlay) overlay.style.transform = 'translate3d(0,' + (y * 0.5) + 'px,0)';
}, { passive: true });
