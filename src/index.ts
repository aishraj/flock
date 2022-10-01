import init, { Universe, BoidLite} from "flock";

//---

init().then( async (raw) => {
  const canvas = document.getElementById(
    "boids-canvas"
  ) as HTMLCanvasElement;



  canvas.height = window.innerHeight; 
  canvas.width = window.innerWidth;




  const NUM_BOIDS = 250;
  const BOID_RADIUS = 100;
  const MAX_SPEED = 3;
  const MAX_FORCE = 0.1;
  const A_COEFF = 0.3;
  const C_COEFF = 0.5;
  const S_COEFF = 0.5;
  const universeHeight = canvas.height;
  const universeWidth = canvas.width;
  const universe = Universe.new(universeHeight, universeWidth, NUM_BOIDS, BOID_RADIUS, A_COEFF, C_COEFF, S_COEFF, MAX_FORCE, MAX_SPEED);

  const ctx = canvas.getContext("2d")!;

  const sizeCanvas = async () => {
    canvas.height = window.innerHeight;
    canvas.width = window.innerWidth;
    universe.set_height(canvas.height);
    universe.set_width(canvas.width);
  };

  window.addEventListener("resize", sizeCanvas, false);
  sizeCanvas();

  const drawBoids = async () => {
    const boids = universe.get_positions();
    ctx.beginPath();
    ctx.fillStyle = "#000000";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < boids.length; i++) {
      const position = boids[i] as BoidLite;
      await drawBoid(position);
    }
    ctx.stroke();

  };

  const isPaused = async () => {
    return animationId === null;
  };


  let animationId: number | null = null;
  const renderLoop = async () => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    await drawBoids();
    universe.tick();
    animationId = requestAnimationFrame(renderLoop);
  };


  const play = async () => {
    await renderLoop();
  };

  const drawBoid = async(boid: BoidLite) => {
    let color = "#" + boid.color.toString(16);
    ctx.fillStyle = color;
    const angle = Math.atan2(boid.vy, boid.vx);
    ctx.translate(boid.x, boid.y);
    ctx.rotate(angle);
    ctx.translate(-boid.x, -boid.y);
    ctx.beginPath();
    ctx.moveTo(boid.x, boid.y);
    ctx.lineTo(boid.x - 15, boid.y + 5);
    ctx.lineTo(boid.x - 15, boid.y - 5);
    ctx.lineTo(boid.x, boid.y);
    ctx.fill();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
  
  }

  const updateMaxSpeed = () => {
	console.log("updating max speed");	
	const maxSpeed = (document.getElementById("boids-max-speed-range") as HTMLInputElement).value;
	universe.set_max_speed(Number(maxSpeed) || 0);
 };

 const updateMaxForce = () => {
	const maxForce = (document.getElementById("boids-max-force-range") as HTMLInputElement).value;
	universe.set_max_force(Number(maxForce) || 0);
 };

  const updateAlignmentCoefficient = () => {
  	const alignmentCoefficient = (document.getElementById("boids-alignment-weight") as HTMLInputElement).value;
  	universe.set_alignment_weight(Number(alignmentCoefficient) || 0);
  };
  
  const updateCohesionCoefficient = () => {
  	const cohesionCoefficient = (document.getElementById("boids-cohesion-weight") as HTMLInputElement).value;
  	universe.set_cohesion_weight(Number(cohesionCoefficient) || 0);
  };
  
  const updateSeparationCoefficient = () => {
  	const separationCoefficient = (document.getElementById("boids-separation-weight") as HTMLInputElement).value;
  	universe.set_separation_weight(Number(separationCoefficient) || 0);
  };
  
  const updateNeighborhoodRadius = () => {
  	const neighborhoodRadius = (document.getElementById("boids-neighbourhood-radius-range") as HTMLInputElement).value;
  	universe.set_search_radius(Number(neighborhoodRadius) || 0);
  };
  
  const speedElement = document.getElementById("boids-max-speed-range") as HTMLInputElement;
  speedElement.addEventListener("input", updateMaxSpeed);
  
  const forceElement = document.getElementById("boids-max-force-range") as HTMLInputElement;
  forceElement.addEventListener("input", updateMaxForce);
  
  const alignmentElement = document.getElementById("boids-alignment-weight") as HTMLInputElement;
  alignmentElement.addEventListener("input", updateAlignmentCoefficient);
  
  const cohesionElement = document.getElementById("boids-cohesion-weight") as HTMLInputElement;
  cohesionElement.addEventListener("input", updateCohesionCoefficient);
  
  const separationElement = document.getElementById("boids-separation-weight") as HTMLInputElement;
  separationElement.addEventListener("input", updateSeparationCoefficient);
  
  const neighborhoodElement = document.getElementById("boids-neighbourhood-radius") as HTMLInputElement;
  neighborhoodElement.addEventListener("input", updateNeighborhoodRadius);

  const boidsWrapper = document.getElementById("boids-wrapper") as HTMLDivElement;

  boidsWrapper.addEventListener("click", async (_event) => {
    if (await isPaused()) {
      play();
    } else {
      pause();
    }
  });

  const pause = () => {
    if (animationId !== null) {
      cancelAnimationFrame(animationId);
    }
    animationId = null;
  };

  await play();
});
