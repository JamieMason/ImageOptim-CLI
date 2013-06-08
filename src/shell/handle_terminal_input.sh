# Handle Inputs
while [ "$1" != "" ]; do
  case $1 in
    -d | --directory )
      shift;
      imgPath=$1
      ;;
    -a | --image-alpha )
      runImageAlpha="true"
      ;;
    -j | --jpeg-mini )
      runJPEGmini="true"
      ;;
    -q | --quit )
      quitOnComplete="true"
      ;;
    -h | --help )
      usage;
      exit 0
      ;;
    -e | --examples )
      examples;
      exit 0
      ;;
    -v | --version )
      version;
      exit 0
      ;;
    * )
    usage
    exit 1
  esac
  shift
done
