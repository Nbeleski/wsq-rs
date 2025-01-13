use std::env;

fn main() {
    let mut builder = cc::Build::new();

    // Add all WSQ and NBIS C source files
    builder
        .include("csrc/commonnbis/include")
        .include("csrc/imgtools/include");

    // Add source directories
    builder
        .files(vec![
            // Add your specific source files here
            "csrc/imgtools/src/lib/jpegl/decoder.c",
            "csrc/imgtools/src/lib/jpegl/encoder.c",
            "csrc/imgtools/src/lib/jpegl/huff.c",
            "csrc/imgtools/src/lib/jpegl/huftable.c",
            "csrc/imgtools/src/lib/jpegl/imgdat.c",
            "csrc/imgtools/src/lib/jpegl/ppi.c",
            "csrc/imgtools/src/lib/jpegl/sd4util.c",
            "csrc/imgtools/src/lib/jpegl/tableio.c",
            "csrc/imgtools/src/lib/jpegl/util.c",

            "csrc/imgtools/src/lib/wsq/cropcoeff.c",
            "csrc/imgtools/src/lib/wsq/decoder.c",
            "csrc/imgtools/src/lib/wsq/encoder.c",
            "csrc/imgtools/src/lib/wsq/globals.c",
            "csrc/imgtools/src/lib/wsq/huff.c",
            "csrc/imgtools/src/lib/wsq/ppi.c",
            "csrc/imgtools/src/lib/wsq/sd14util.c",
            "csrc/imgtools/src/lib/wsq/tableio.c",
            "csrc/imgtools/src/lib/wsq/tree.c",
            "csrc/imgtools/src/lib/wsq/util.c",
            
            "csrc/commonnbis/src/lib/fet/allocfet.c",
            "csrc/commonnbis/src/lib/fet/delfet.c",
            "csrc/commonnbis/src/lib/fet/extrfet.c",
            "csrc/commonnbis/src/lib/fet/freefet.c",
            "csrc/commonnbis/src/lib/fet/lkupfet.c",
            "csrc/commonnbis/src/lib/fet/nistcom.c",
            "csrc/commonnbis/src/lib/fet/printfet.c",
            "csrc/commonnbis/src/lib/fet/readfet.c",
            "csrc/commonnbis/src/lib/fet/strfet.c",
            "csrc/commonnbis/src/lib/fet/updatfet.c",
            "csrc/commonnbis/src/lib/fet/writefet.c",

            "csrc/commonnbis/src/lib/ioutil/dataio.c",
            "csrc/commonnbis/src/lib/ioutil/filesize.c",
            
            "csrc/commonnbis/src/lib/util/bres.c",
            "csrc/commonnbis/src/lib/util/bubble.c",
            "csrc/commonnbis/src/lib/util/computil.c",
            "csrc/commonnbis/src/lib/util/fatalerr.c",
            "csrc/commonnbis/src/lib/util/invbyte.c",
            "csrc/commonnbis/src/lib/util/invbytes.c",
            "csrc/commonnbis/src/lib/util/memalloc.c",
            "csrc/commonnbis/src/lib/util/ssxstats.c",
            "csrc/commonnbis/src/lib/util/syserr.c",
            "csrc/commonnbis/src/lib/util/ticks.c",
            "csrc/commonnbis/src/lib/util/time.c",
        ]);

    // Add the necessary compile flags
    builder.define("_POSIX_SOURCE", None);  // Equivalent to -D_POSIX_SOURCE
    builder.define("__NBISLE__", None);     // Equivalent to -D__NBISLE__
    builder.define("__NBIS_PNG__", None);   // Equivalent to -D__NBIS_PNG__

    // Specify the architecture flag (e.g., -m64 for 64-bit)
    builder.flag("-m64");

    // Warning flags for nbis code, not working :|
    // builder.flag("-Wunused-parameter");
    // builder.flag("-Wcpp");
    // builder.flag("-Wsign-compare");
    // builder.flag("-Wmisleading-indentation");
    

    // Enable compiler settings, e.g., debug mode
    if env::var("DEBUG").unwrap_or_default() == "true" {
        builder.define("DEBUG", None);
    }

    // Compile and link to your Rust library
    builder.compile("nbis_wsq");
}

