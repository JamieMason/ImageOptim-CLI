# Handle Inputs
while [ "$1" != "" ]; do
  case $1 in
    -d | --directory | -f | --file )
      if [[ "$1" == *"d"* ]]; then
        runMode="directory"
      else
        runMode="file"
      fi
      shift;
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
