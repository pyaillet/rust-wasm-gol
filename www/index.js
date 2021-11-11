import * as wasm from "rust-wasm-gol";

let ctx = wasm.init();
let simu = wasm.create_simulation();
simu.draw_grid(ctx);

simu.fill_random();
simu.draw_cells(ctx);

let next = function(){ 
  simu.next_turn(ctx);
  simu.debug_grid();
  setTimeout(next, 800);
};
next();
