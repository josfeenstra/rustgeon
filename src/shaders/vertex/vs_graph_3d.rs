pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute float aY;

    uniform mat4 uProjection;

    varying lowp vec4 vColor;

    void main() {
        gl_Position = uProjection * vec4(aPosition.x, aY, aPosition.z, 1.0);
        vColor = vec4((aY + 1.0) / 2.0, 0.5, 0.5, 1.0);
    }
    
"#;