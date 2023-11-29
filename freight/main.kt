import kotlin.getOrThrow
// import kotlinx.serialization.Serializable

// @Serializable
class YumaCache() {

}

val Cache: YumaCache by lazy {
        TODO()
};

fun main(args: Array<String>) {
    when (args[0]) {
        "init" -> {
            // sh: cargo init
            // sh: echo "yuma = { package = \"yuma\", git = \"https://github.com/Focus172/freight.git\" }" >> Cargo.toml
        }
        "add" -> {
            // update the cache file to add the thing and rebuild
        }
        "list" -> {

            }
    }
    // "list")
    //     cat $YUMACACHE | jq '.enabled_packages' | tail +2 | head -n-1 | sed 's/  //g;s/"//g;s/,//g'
    //     ;;
    // "help")
    //     usage
    // ;;
    // *)
    //     echo "Unknown Option: { $1 }"
    //     echo ""
    //     usage
    //     exit 1
    // ;;
}

val YUMACACHE: String by lazy {
    "/home/focus/.config/iyuma/.yumacache.json"
};

class Backend(name: String);

fun find_backend(): Backend {
    return Backend("paru");
}

fun pkg(backend: Backend) {
    when (backend) {
        Backend("paru") -> {}
        Backend("brew") -> {}
        else -> {}
    }
}

val USAGE: String = "Freight - A code based package manager
freight [subcommand]

subcommands:
init
    create a new system declaration with sensible defaults

list
    list the installed packages

add
    reserved word

help
    show this text";
