const rustlibs = ["ctxkit"]
const juliapackage = "TxKit"

function build_dylib()
    # Find out the release directory for the distribution
    p = joinpath(@__DIR__, "../../target/debug")
    release_dir = if haskey(ENV, "TXKIT_LIBDIR")
                      ENV["TXKIT_LIBDIR"]
                  elseif isdir(p)
                      p
                  else
                      error("TXKIT_LIBDIR not set, and not running from a source distribution")
                  end

    # Copy libraries to the target directory
    for lib in rustlibs
        # File name for the library
        dylib = dylib_filename(lib)
        # Path to the source library file
        release_dylib_filepath = joinpath(release_dir, dylib)
        # Check the source exists
        @assert isfile(release_dylib_filepath) "$release_dylib_filepath not found. Build may have failed."
        # Copy to the Julia package directory
        cp(release_dylib_filepath, joinpath(@__DIR__, dylib), force=true)
    end

    # Write the loader script
    write_deps_file(map(x -> "lib" * replace(x, '-' => '_'), rustlibs), map(dylib_filename, rustlibs), juliapackage)
end

function dylib_filename(libname)
    libname = replace(libname, '-' => '_')

    @static if Sys.isapple()
        "lib$libname.dylib"
    elseif Sys.islinux()
        "lib$libname.so"
    elseif Sys.iswindows()
        "$libname.dll"
    else
        error("Not supported: $(Sys.KERNEL)")
    end
end

function write_deps_file(libnames, libfiles, juliapackage)
    script = """
import Libdl

"""

    for (libname, libfile) in zip(libnames, libfiles)
        println(libname)
        println(libfile)
        script *= "const $libname = joinpath(@__DIR__, \"$libfile\")\n"
    end

    script *= """

function check_deps()
    global """

    script *= libnames[1]
    for libname in libnames[2:length(libnames)]
        script *= ", $libname"
    end
    script *= "\n"

    for libname in libnames
        script *= """

    if Libdl.dlopen_e($libname) == C_NULL
        error("\$$libname cannot be opened, Please re-run Pkg.build(\\"$juliapackage\\"), and restart Julia.")
    end
"""
    end

    script *= """
end
"""

    open(joinpath(@__DIR__, "deps.jl"), "w") do f
        write(f, script)
    end
end

build_dylib()

# vim: ft=julia:sw=4:ts=4:et
