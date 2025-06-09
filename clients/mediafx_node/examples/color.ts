#!/usr/bin/env node
// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

const { MediaFX } = require("../index.js");

const client = new MediaFX();
// const [width, height] = client.frameSize;
const renderedFrame = new Uint8Array(client.frameBytecount);

while (true) {
  let [, r, g, b] = client.renderFrame();
  r = Math.floor(r * 255);
  g = Math.floor(g * 255);
  b = Math.floor(b * 255);
  for (let offset = 0; offset < renderedFrame.length; offset += 4) {
    renderedFrame[offset] = r;
    renderedFrame[offset + 1] = g;
    renderedFrame[offset + 2] = b;
    renderedFrame[offset + 3] = 255;
  }
  client.renderCommit(renderedFrame);
}
