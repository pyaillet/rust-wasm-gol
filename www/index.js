import * as wasm from "rust-wasm-gol";

let ctx = wasm.init();
let simu = wasm.create_simulation();
simu.draw_grid(ctx);

simu.fill_random();
simu.draw_cells(ctx);

let next = function(){ 
  simu.next_turn(ctx);
  console.log(simu.to_string());
  console.log(simu.turn);
  setTimeout(next, 800);
};
next();
