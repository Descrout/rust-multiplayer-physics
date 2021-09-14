const address = `ws://192.168.1.106:6464`;

let canvas, scaleMultiplier, mouse, socket;

let connectionSuccess = false;

let state;

class Parser {
    static serialize(obj) {
        let pbf = new Pbf();
        pbf.writeBytes([0]); // we only have one header so i just use 0
        GameInput.write(obj, pbf);
        return pbf.finish().slice(1);
    }

    static deSerialize(data) {
        let pbf = new Pbf(data.slice(1));
        switch (data[0]) {
            case 0: return State.read(pbf);
            default:
                console.error("Receive header doesn't match");
                return;
        }
    }
}

function scaleToWindow(canvas, backgroundColor) {
	let scaleX, scaleY, scale, center;

	scaleX = window.innerWidth / canvas.offsetWidth;
	scaleY = window.innerHeight / canvas.offsetHeight;

	scale = Math.min(scaleX, scaleY);
	canvas.style.transformOrigin = "0 0";
	canvas.style.transform = "scale(" + scale + ")";

	if (canvas.offsetWidth > canvas.offsetHeight) {
		if (canvas.offsetWidth * scale < window.innerWidth) {
			center = "horizontally";
		} else {
			center = "vertically";
		}
	} else {
		if (canvas.offsetHeight * scale < window.innerHeight) {
			center = "vertically";
		} else {
			center = "horizontally";
		}
	}

	let margin;
	if (center === "horizontally") {
		margin = (window.innerWidth - canvas.offsetWidth * scale) / 2;
		canvas.style.marginTop = 0 + "px";
		canvas.style.marginBottom = 0 + "px";
		canvas.style.marginLeft = margin + "px";
		canvas.style.marginRight = margin + "px";
	}

	if (center === "vertically") {
		margin = (window.innerHeight - canvas.offsetHeight * scale) / 2;
		canvas.style.marginTop = margin + "px";
		canvas.style.marginBottom = margin + "px";
		canvas.style.marginLeft = 0 + "px";
		canvas.style.marginRight = 0 + "px";
	}

	canvas.style.paddingLeft = 0 + "px";
	canvas.style.paddingRight = 0 + "px";
	canvas.style.paddingTop = 0 + "px";
	canvas.style.paddingBottom = 0 + "px";
	canvas.style.display = "block";

	document.body.style.backgroundColor = backgroundColor;

	let ua = navigator.userAgent.toLowerCase();
	if (ua.indexOf("safari") != -1) {
		if (ua.indexOf("chrome") > -1) {
			// Chrome
		} else {
			// Safari
			//canvas.style.maxHeight = "100%";
			//canvas.style.minHeight = "100%";
		}
	}

	return scale;
}

function lerp(a, b, t) {
	return a + (b - a) * t;
}