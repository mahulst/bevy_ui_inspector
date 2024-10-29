// gradient_shader.wgsl


@fragment
fn fragment(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = pos.xy / vec2<f32>(1920.0, 1080.0);
    let hue = 20.0;
    // Calculate saturation and lightness
    let saturation = uv.x;
    let lightness = 0.5 - uv.y / 2.0;

    // Convert HSL to RGB
    let color = hsl_to_rgb(hue, saturation, lightness);
    // return vec4<f32>(1.0,0.0,0.0,1.0);
    return vec4<f32>(color, 1.0);
}

fn hue2rgb(p: f32, q: f32, t: f32) -> f32 {
    var tt = t;
    if tt < 0.0 {
        tt = tt + 1.0;
    }
    if tt > 1.0 {
        tt = tt - 1.0;
    }
    if tt < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * tt;
    } else if tt < 0.5 {
        return q;
    } else if tt < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - tt) * 6.0;
    } else {
        return p;
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    var r: f32;
    var g: f32;
    var b: f32;

    // Normalize hue to [0, 1]
    let h_norm = h / 360.0;

    if s == 0.0 {
        // Achromatic color (gray)
        r = l;
        g = l;
        b = l;
    } else {
        var q: f32 = 0;
          if l < 0.5 {
            q = l * (1.0 + s);
        } else {
            q = l + s - l * s;
        };
        let p = 2.0 * l - q;
        r = hue2rgb(p, q, h_norm + 1.0 / 3.0);
        g = hue2rgb(p, q, h_norm);
        b = hue2rgb(p, q, h_norm - 1.0 / 3.0);
    }

    return vec3<f32>(r, g, b);
}
