#!/usr/bin/env node
// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

const { MediaFX } = require("../index.js");

const client = new MediaFX();
const [width, height] = client.frameSize;
const renderedFrame = new Uint8Array(client.frameBytecount);
const frames = [new Uint8Array(client.frameBytecount)];

while (true) {
  let [, xshift, yshift] = client.renderFrame(frames);

  xshift = Math.floor(xshift * width);
  yshift = Math.floor(yshift * height);
  for (let dy = 0; dy < height; dy++) {
    for (let dx = 0; dx < width; dx++) {
      const sy = (dy + yshift) % height;
      const sx = (dx + xshift) % width;
      const destIndex = (dy * width + dx) * 4;
      const sourceIndex = (sy * width + sx) * 4;
      for (let channel = 0; channel < 4; channel++) {
        renderedFrame[destIndex + channel] = frames[0][sourceIndex + channel];
      }
    }
  }

  client.renderCommit(renderedFrame);
}
