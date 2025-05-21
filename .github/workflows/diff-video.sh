#!/usr/bin/env bash
# Copyright (C) 2023 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later
usage="$0 <original-video> <new-video> <output.nut>"

# https://superuser.com/questions/854543/how-to-compare-the-difference-between-2-videos-color-in-ffmpeg
read -r -d '' FILTER <<'EOF'
[0:a]asplit[org1a][org2a];
[org1a]showwaves=s=320x100:mode=line:rate=25[orgwaveform];
[0:v]pad=iw+max(320-iw\,0):ih+100[pad],[pad][orgwaveform]overlay=0:H-100,split[org1v][org2v];
[1:a]asplit[new1a][new2a];
[new1a]showwaves=s=320x100:mode=line:rate=25[newwaveform];
[1:v]pad=iw+max(320-iw\,0):ih+100[pad],[pad][newwaveform]overlay=0:H-100,split[new1v][new2v];
[org1v][new1v]blend=all_mode=grainextract[blendv];
[org2a][new2a]amix;
[org2v][new2v][blendv]vstack=inputs=3
EOF

ffmpeg -hide_banner -i "${1:?$usage}" -i "${2:?$usage}" -filter_complex "$FILTER" -f nut -codec:a pcm_f32le -codec:v ffv1 -flags bitexact -g 1 -level 3 -pix_fmt rgb32 -fflags bitexact -y "${3:?$usage}"
