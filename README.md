# LPC178x7x-pac

This repository contains low-level register mappings for the NXP LPC177x and LPC178x
series microcontrollers. The code is generated from an SVD file using svd2rust.

The SVD file used in this project is **heavily edited and unofficial** as the original was unusable
with svd2rust, though only the naming has been modified, not the bit positions or addresses.

## Documentation

The latest original SVD files are available in the [LPC1700_DFP CMSIS-Pack](https://www.keil.arm.com/packs/lpc1700_dfp-keil/devices/).

Additional vendor supplied documents:
- [Datasheet](https://www.nxp.com/docs/en/data-sheet/LPC178X_7X.pdf)
- [User manual](https://www.nxp.com/webapp/Download?colCode=UM10470)
- [Errata](https://www.nxp.com/docs/en/errata/ES_LPC177X_8X.pdf)