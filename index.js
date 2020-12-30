const wasm = import('./pkg/');
const canvas = document.getElementById('wasmcanvas');
const gl = canvas.getContext('webgl', {antialias: true});

wasm.then(rust => 
{
    rust.welcome_message();

    // rendering
    if (!gl) {
        alert('failed to initialize webgl');
        return;
    }

    // geon
    const client = new rust.Client();    

    // timing
    const FPS_THROTTLE = 1000.0 / 144.0; // miliseconds per frame
    let lastDrawTime = -1; // in miliseconds
    const initialTime = Date.now();
    
    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();
        

        if (currTime >= lastDrawTime + FPS_THROTTLE)
        {
            // update time 
            lastDrawTime = currTime;

            // resize if needed
            if (window.innerHeight != canvas.height || 
                window.innerWidth  != canvas.width) 
            {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width  = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let et = currTime - initialTime;
            client.update(et, window.innerHeight, window.innerWidth);
            client.draw();
        }
    }

    render();
}).catch(console.error);