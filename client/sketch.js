function setup() {
	canvas = createCanvas(960, 540);
	canvas.elt.addEventListener('contextmenu', event => event.preventDefault());
	scaleMultiplier = scaleToWindow(canvas.elt);

	mouse = {x: 0, y: 0};
	
	noLoop();

	socket = new Network(address);

	let promise = socket.connect(received);
	if (promise) {
        promise.then(() => {
			loop();
        }).catch((err) => {
            alert("Server is offline !");
        });
    }
}

function draw() {
	let dt = deltaTime / 1000;
	if (dt > 0.033) dt = 0.033;
	if(touches.length) {
		mouse.x = touches[0].x / scaleMultiplier;
		mouse.y = touches[0].y / scaleMultiplier;
		mouseIsPressed = true;
	}else {
		mouse.x = mouseX / scaleMultiplier;
		mouse.y = mouseY / scaleMultiplier;
	}
	
	update(dt);
	
	// Draw
	background(255);
	if(!state) return;

	stroke(0);
	strokeWeight(1);
	for(const body of state.bodies) {
		push();
		fill(body.color);
		const half_w = body.w * 50;
		const half_h = body.h * 50;
		const x = body.x * 50 - half_w;
		const y = height - (body.y * 50 + half_h);
		translate(half_w + x, half_h + y);
		rotate(-body.rotation);
		rect(-half_w, -half_h, half_w * 2, half_h * 2);
		pop();
	}

	strokeWeight(2);
	noStroke();
	for(const player of state.entities) {
		if(player.pressed) stroke(0);
		else noStroke();
		fill(player.color);
		circle(player.x, player.y, 10);
	}
}

function update(dt) {
	if(!connectionSuccess) return;
	socket.send({
		x: mouse.x,
		y: mouse.y,
		pressed: mouseIsPressed
	});
}

function received(header, obj) {
	state = obj;
}

function windowResized() {
	scaleMultiplier = scaleToWindow(canvas.elt);
}