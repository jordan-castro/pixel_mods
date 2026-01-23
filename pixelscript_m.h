#ifndef PIXEL_SCRIPT_M_H
#define PIXEL_SCRIPT_M_H

// Helpful to not have to write out the method everytime.
#define PXS_HANDLER(name) pxs_Var* ##name(pxs_Argc argc, pxs_Argv argv, pxs_Opaque opaque)

#endif // PIXEL_SCRIPT_M_H