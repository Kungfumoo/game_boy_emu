INCLUDE "hardware.inc/hardware.inc"
INCLUDE "header.inc"


SECTION "Boot ROM", ROM0[$000]

EntryPoint:
    0x31: ld sp, hStackBottom

    0xaf: xor a
    0x21: ld hl, $9FFF
.clearVRAM
    0x32: ld [hld], a
    0xcb7C: bit 7, h
    0x20: jr nz, .clearVRAM

    0x21: ld hl, rNR52
    0x0E: ld c, LOW(rNR11) ; CH1 length
    ; Enable APU
    ; This sets (roughly) all audio registers to 0
    0x3E: ld a, $80
    0x32: ld [hld], a
    ; hl = rNR51
    ; Set CH1 duty cycle to 25%
    0xE2: ldh [c], a
    0x0C: inc c ; ld c, LOW(rNR11) ; CH1 envelope
    0x3E: ld a, $F3 ; Initial volume 15, 3 decreasing sweep
    0xE2: ldh [c], a
    ; Route all channels to left speaker, CH2 and CH1 to right speaker
    0x32: ld [hld], a
    ; hl = rNR50
    ; Set volume on both speakers to 7, disable VIN on both speakers
    0x3E: ld a, $77
    0x77: ld [hl], a

    0x3E: ld a, $FC
    0xE0: ldh [rBGP], a

IF DEF(dmg0)
    ld hl, HeaderLogo
    push hl
    ld de, Logo
.checkLogo
    ld a, [de]
    inc de
    cp [hl]
    jr nz, Lockup
    inc hl
    ld a, l
    cp LOW(HeaderTitle)
    jr nz, .checkLogo
    ld b, HeaderChecksum - HeaderTitle
    ld a, b
.computeChecksum
    add a, [hl]
    inc hl
    dec b
    jr nz, .computeChecksum
    add a, [hl]
    jr nz, Lockup
    pop de ; ld de, HeaderLogo
ELSE
    0x11: ld de, HeaderLogo
ENDC
    0x21: ld hl, vLogoTiles
.decompressLogo
    0x1A: ld a, [de]
    0xCD: call DecompressFirstNibble
    0xCD: call DecompressSecondNibble
    0x13: inc de
    0x7B: ld a, e
    0xFE: cp LOW(HeaderTitle)
    0x20: jr nz, .decompressLogo

IF !DEF(dmg0)
    ; ld hl, vRTile
    0x11: ld de, RTile
    0x06: ld b, 8
.copyRTile
    0x1A: ld a, [de]
    0x13: inc de
    0x22: ld [hli], a
    0x23: inc hl ; Skip every other byte
    0x05: dec b
    0x20: jr nz, .copyRTile
    0x3E: ld a, $19
    0xEA: ld [vMainTilemap + SCRN_VX_B * 8 + 16], a ; 8 rows down, 16 across
ELSE
    ld a, $18
ENDC

    0x21: ld hl, vMainTilemap + SCRN_VX_B * 9 + 15
.writeTilemapRow
    0x0E: ld c, 12
.writeTilemapByte
IF DEF(dmg0)
    ld [hld], a
ENDC
    0x3D: dec a
    0x28: jr z, ScrollLogo
IF !DEF(dmg0)
    0x32: ld [hld], a
ENDC
    0x0D: dec c
    0x20: jr nz, .writeTilemapByte
IF DEF(dmg0)
    ; Go to previous row
    ld de, -(SCRN_VX_B - 12)
    add hl, de
ELSE
    ld l, LOW(vMainTilemap + SCRN_VX_B * 8 + 15)
ENDC
    jr .writeTilemapRow


ScrollLogo:
    ; a = 0
    0x67: ld h, a ; ld h, 0
    0x3E: ld a, $64
    0x57: ld d, a
    0xE0: ldh [rSCY], a
    0x3E: ld a, LCDCF_ON | LCDCF_BLK01 | LCDCF_BGON
    0xE0: ldh [rLCDC], a
    0x04: inc b ; ld b, 1

    ; h = Number of times the logo was scrolled up
    ; d = How many frames before exiting the loop
    ; b = Whether to scroll the logo

.loop
    0x1E: ld e, 2
IF DEF(dmg0)
    call DelayFrames
ELSE
.delayFrames
    0x0E: ld c, 12
.waitVBlank
    0xF0: ldh a, [rLY]
    0xFE: cp SCRN_Y
    0x20: jr nz, .waitVBlank ; IT IS LOOPING INDEFINATELY HERE!!!
    0x0D: dec c
    jr nz, .waitVBlank
    dec e
    jr nz, .delayFrames
ENDC

    ld c, LOW(rNR13) ; CH1 frequency low byte
    inc h
    ld a, h
    ld e, $83
    cp $62
    jr z, .playSound
    ld e, $C1
    cp $64
    jr nz, .dontPlaySound
.playSound
    ld a, e
    ldh [c], a
    inc c ; ld c, LOW(rNR14) ; CH1 frequency high byte
    ; Set frequency to $7XX and restart channel
    ld a, $87
    ldh [c], a
.dontPlaySound
    ldh a, [rSCY]
    sub b
    ldh [rSCY], a
    dec d
    jr nz, .loop

    dec b
IF DEF(dmg0)
    jr nz, Done
ELSE
    jr nz, CheckLogo
ENDC
    ld d, $20
    jr .loop


IF DEF(dmg0)
Lockup:
    ld a, LCDCF_ON | LCDCF_BLK01 | LCDCF_BGON
    ldh [rLCDC], a
.loop
    ld e, 20
    call DelayFrames
    ldh a, [rBGP]
    xor a, $FF
    ldh [rBGP], a
    jr .loop
ENDC


DecompressFirstNibble:
    0x4F: ld c, a
DecompressSecondNibble:
    0x06: ld b, 8 / 2 ; Set all 8 bits of a, "consuming" 4 bits of c
.loop
    0xC5: push bc
    0XCB11: rl c ; Extract MSB of c
    0x17: rla ; Into LSB of a
    0xC1: pop bc
    0xCB11: rl c ; Extract that same bit
    0x17: rla ; So that bit is inserted twice in a (= horizontally doubled)
    0x05: dec b
    0x20: jr nz, .loop
    0x22: ld [hli], a
    0x23: inc hl ; Skip second plane
    0x22: ld [hli], a ; Also double vertically
    0x23: inc hl
    0xC9: ret


IF DEF(dmg0)
DelayFrames:
    ld c, 12
.loop
    ldh a, [rLY]
    cp SCRN_Y
    jr nz, .loop
    dec c
    jr nz, .loop
    dec e
    jr nz, DelayFrames
    ret
ENDC


; Each tile is encoded using 2 (!) bytes
; How to read: the logo is split into two halves (top and bottom), each half being encoded
;              separately. Each half must be read in columns.
;              So, the first byte is `db %XX.._XXX.`, then `db %XXX._XX.X`, matching the
;              `db $CE, $ED` found in many places. And so on! :)
MACRO logo_row_gfx
    ASSERT _NARG % 4 == 0
    PUSHO
    OPT b.X
    FOR N1, 1, _NARG / 4 + 1 ; N1, N2, N3, and N4 iterate through the 4 equally-sized rows
        DEF N2 = N1 + _NARG / 4
        DEF N3 = N2 + _NARG / 4
        DEF N4 = N3 + _NARG / 4
        db %\<N1>\<N2>, %\<N3>\<N4>
    ENDR
    POPO
ENDM

; Whitespace is not stripped after line continuations until RGBDS v0.6.0, so rows are not indented
    Logo:  logo_row_gfx \
XX.., .XX., XX.., ...., ...., ...., ...., ...., ...., ...X, X..., ...., \
XXX., .XX., XX.., ...., ..XX, ...., ...., ...., ...., ...X, X..., ...., \
XXX., .XX., ...., ...., .XXX, X..., ...., ...., ...., ...X, X..., ...., \
XX.X, .XX., XX.X, X.XX, ..XX, ..XX, XX.., XX.X, X..., XXXX, X..X, XXX.
           logo_row_gfx \
XX.X, .XX., XX.X, XX.X, X.XX, .XX., .XX., XXX., XX.X, X..X, X.XX, ..XX, \
XX.., XXX., XX.X, X..X, X.XX, .XXX, XXX., XX.., XX.X, X..X, X.XX, ..XX, \
XX.., XXX., XX.X, X..X, X.XX, .XX., ...., XX.., XX.X, X..X, X.XX, ..XX, \
XX.., .XX., XX.X, X..X, X.XX, ..XX, XXX., XX.., XX.., XXXX, X..X, XXX.


IF !DEF(dmg0)
RTile:
    PUSHO
    OPT b.X
    db %..XXXX..
    db %.X....X.
    db %X.XXX..X
    db %X.X..X.X
    db %X.XXX..X
    db %X.X..X.X
    db %.X....X.
    db %..XXXX..
    POPO


CheckLogo:
    ld hl, HeaderLogo
    ld de, Logo
.compare
    ld a, [de]
    inc de
    cp [hl]
.logoFailure
    jr nz, .logoFailure
    inc hl
    ld a, l
    cp LOW(HeaderTitle)
    jr nz, .compare

    ld b, HeaderChecksum - HeaderTitle
    ld a, b
.computeChecksum
    add a, [hl]
    inc hl
    dec b
    jr nz, .computeChecksum
    add a, [hl]
.checksumFailure
    jr nz, .checksumFailure

    IF DEF(mgb)
    ld a, $FF
    ELSE
    ld a, 1
    ENDC

ELSE
    ds 2
Done:
    inc a
ENDC
    ldh [$FF50], a
    assert @ == $100 ; Execution now falls through to the cartridge's header



SECTION "VRAM tiles", VRAM[$8000],BANK[0]

vBlankTile:
    ds $10
vLogoTiles:
    ds $10 * (HeaderTitle - HeaderLogo) / 2
vRTile:
    ds $10

SECTION "VRAM tilemap", VRAM[$9800],BANK[0]

vMainTilemap:
    ds SCRN_VX_B * SCRN_VY_B


SECTION "HRAM", HRAM[$FFEE]

    ds $10
hStackBottom: