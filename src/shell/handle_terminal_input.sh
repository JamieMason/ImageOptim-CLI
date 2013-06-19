# Handle Inputs
while [ "$1" != "" ]; do
  case $1 in
    -d | --directory )
      shift;
      runMode="directory"
      imgPath=$1
      ;;
    -a | --image-alpha )
      useImageAlpha="true"
      ;;
    -j | --jpeg-mini )
      useJPEGmini="true"
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
      echo $version;
      exit 0
      ;;
    * )
    usage
    exit 1
  esac
  shift
done
