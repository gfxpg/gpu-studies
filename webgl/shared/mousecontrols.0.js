export default class MouseControls {
  constructor(canvas, rotationHandler) {
    this.canvas = canvas;
    this.prevX = 0;
    this.prevY = 0;
    this.currRadX = 0;
    this.currRadY = 0;
    this.mouseDown = false;
    this.onMouseMove = this.onMouseMove.bind(this);
    this.rotationHandler = rotationHandler;

    canvas.addEventListener('mousedown', (e) => {
      this.mouseDown = true;

      const { x, y } = MouseControls.readMousePosition(e, canvas);
      this.prevX = x;
      this.prevY = y;
    }, false);

    canvas.addEventListener('mouseup', () => {
      this.mouseDown = false;
    }, false);

    canvas.addEventListener("mousemove", this.onMouseMove, false);
  }

  onMouseMove(e) {
    if (!this.mouseDown) return;

    const { x, y } = MouseControls.readMousePosition(e, this.canvas); 

    const dx = x - this.prevX,
          dy = y - this.prevY;

    const dradX = 2 * Math.PI / this.canvas.width * dx,
          dradY = 2 * Math.PI / this.canvas.height * dy;

    this.currRadX += dradX;
    this.currRadY += dradY;

    this.prevX = x;
    this.prevY = y;

    this.rotationHandler(this.currRadY, this.currRadX);
  }

  static readMousePosition(e, canvas) {
    const rect = canvas.getBoundingClientRect();

    return {
      x: (e.clientX - rect.left) / (rect.right - rect.left) * canvas.width,
      y: (e.clientY - rect.top) / (rect.bottom - rect.top) * canvas.height
    };
  }
}
