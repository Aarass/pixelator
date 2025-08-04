import { $ } from "zx";

for (let i = 0; i < 255; i += 3) {
  await $`/home/aaras/dev/pixelator/target/debug/pixelator ~/Downloads/large_65b0ae90317f7596d6f95bfdd6131398.jpg -o ~/Downloads/output/output${i / 3}.jpg -s ${Math.floor(8 + (i * 2) / 255)} -t ${Math.floor(i * 0.7)}`;
}

// ➜  ffmpeg -framerate 10 -i output%d.jpg output.mp4
// ➜  bun run ./scripts/threshold.mts
