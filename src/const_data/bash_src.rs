pub const DEFAULT_EXE_BUILD_SH: &str = 
r#"#!/bin/bash

### COLORS ###

RED="\033[1;31m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
MAGENTA="\033[1;35m"
CYAN="\033[1;36m"
GREY="\033[0;37m"
NOCOLOR="\033[m"

##############

CMAKE_BUILD_DIR='./bin'
PROJECT_NAME='{{project_name}}'
BUILD_TYPE=Debug
POSTFIX='-d'

configure() {
	echo -e "Configuring CMake..."
	cmake -G Ninja -DCMAKE_BUILD_TYPE=$BUILD_TYPE -S . -B $CMAKE_BUILD_DIR
    echo -e "CMake configured."
}

build() {
	echo -e "Building..."
	{ cmake --build $CMAKE_BUILD_DIR --config $BUILD_TYPE --verbose; } \
	|| { configure && cmake --build $CMAKE_BUILD_DIR --config $BUILD_TYPE --verbose; } \
	|| { echo -e "${RED}Building failure${NOCOLOR}"; false; }
}

run() {
	echo -e "Running..."
    $CMAKE_BUILD_DIR/$PROJECT_NAME$POSTFIX
}

build_and_run() {
	{ build && run; } || echo -e "${RED}Build&Run failed${NOCOLOR}"
}

clean_all() {
	echo -e "Cleaning..."
	rm -rf $CMAKE_BUILD_DIR/{*,.*} &> /dev/null
	echo -e "${GREEN}All clean!${NOCOLOR}"
}

change_build_type() {
	echo -e "\nBuild type -> ${GREEN}${BUILD_TYPE}${NOCOLOR}"
	echo -e "Choose build type:\n (${RED}d${NOCOLOR})Debug, (${RED}r${NOCOLOR})Release"
	read -n 1 -s input
	case $input in
		d)
			BUILD_TYPE=Debug
			POSTFIX='-d'
			;;
		r)
			BUILD_TYPE=Release
			POSTFIX=''
			;;
		*)
			;;
	esac
}

##### Script Start #####

while true
do
echo -e "\n \
Build type -> ${GREEN}${BUILD_TYPE}${NOCOLOR}\n \
(${RED}1${NOCOLOR}) configure cmake, \n \
(${RED}2${NOCOLOR}) build, \n \
(${RED}3${NOCOLOR}) build & run, \n \
(${RED}4${NOCOLOR}) run, \n \
(${RED}5${NOCOLOR}) clean all, \n \
(${RED}b${NOCOLOR}) change build type, \n \
(${GREEN}q${NOCOLOR}) exit\
"

read -n 1 -s input
case $input in
	1)
		configure
		;;
	2)
		build
		;;
	3)
		build_and_run
		;;
	4)
		run
		;;
	5)
		clean_all
		;;
	b)
		change_build_type
		;;
	*)
		exit
		;;
esac
done
"#;

pub const DEFAULT_LIB_BUILD_SH: &str = 
r#"#!/bin/bash

### COLORS ###

RED="\033[1;31m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
MAGENTA="\033[1;35m"
CYAN="\033[1;36m"
GREY="\033[0;37m"
NOCOLOR="\033[m"

##############


CMAKE_BUILD_DIR='./bin'
PROJECT_NAME='{{project_name}}'
LIB_EXTENSION='dll'
BUILD_TYPE=Debug
POSTFIX='-d'
TEST_PROJECT_PATH="test_project"
TEST_PROJECT_LIB_PATH="$TEST_PROJECT_PATH/external/lib"
TEST_PROJECT_INCLUDE_PATH="$TEST_PROJECT_PATH/external/{{project_name}}/include"
INCLUDE_EXPORT_DIR="include"

compile_include_files() {
	# path to manually compiled external lib headers
	EXTERNAL_HEADER_DIR="external/include_export"
	# path to internal source files
	INTERNAL_SOURCE_DIR="src"

	echo "Compiling include files folder..."

	rm -rf $INCLUDE_EXPORT_DIR
	mkdir -p $INCLUDE_EXPORT_DIR

    if [[ -d $EXTERNAL_HEADER_DIR ]]; then
	    # copy external header files
	    cp -r $EXTERNAL_HEADER_DIR/* $INCLUDE_EXPORT_DIR
    fi

	# copy all internal source files
	cp -r $INTERNAL_SOURCE_DIR/* $INCLUDE_EXPORT_DIR
	# remove all internal .cpp files
	find $INCLUDE_EXPORT_DIR -type f -name "*.cpp" -delete

	echo "Include files folder compiled ($PWD/$INCLUDE_EXPORT_DIR)"
}

determine_lib_extension() {
	if [[ $(uname -s) == "Linux" ]]; then
		LIB_EXTENSION='so'
	else
		LIB_EXTENSION='dll'
	fi
}

copy_lib_to_test_project() {
	# create lib dir for test project if it doesn't exist
	mkdir -p $TEST_PROJECT_LIB_PATH

	# for some reason on linux "bin" directory isn't created
	LIB_PATH_WINDOWS="$CMAKE_BUILD_DIR/bin/$PROJECT_NAME$POSTFIX.$LIB_EXTENSION"
	LIB_PATH_LINUX="$CMAKE_BUILD_DIR/$PROJECT_NAME$POSTFIX.$LIB_EXTENSION"
	if test -f $LIB_PATH_WINDOWS; then
		cp $LIB_PATH_WINDOWS $TEST_PROJECT_LIB_PATH/
	elif test -f $LIB_PATH_LINUX; then
		cp $LIB_PATH_LINUX $TEST_PROJECT_LIB_PATH/
	else
		echo "Wasn't able to find library file"
		return 1
	fi
	rm -rf $TEST_PROJECT_INCLUDE_PATH
	mkdir -p $TEST_PROJECT_INCLUDE_PATH
	cp -r $INCLUDE_EXPORT_DIR/* $TEST_PROJECT_INCLUDE_PATH/
}

build_test_project() {
	echo -e "Building test project..."
	cd $TEST_PROJECT_PATH
	echo 5 | source ./build.sh
	echo 2 | source ./build.sh
	cd -
}

configure() {
	echo -e "Configuring Cmake..."
	cmake -G Ninja -DCMAKE_BUILD_TYPE=$BUILD_TYPE -S . -B $CMAKE_BUILD_DIR
}

build() {
	echo -e "Building..."
	compile_include_files
	{ cmake --build $CMAKE_BUILD_DIR --config $BUILD_TYPE --verbose && copy_lib_to_test_project; } \
	|| { configure && cmake --build $CMAKE_BUILD_DIR --config $BUILD_TYPE --verbose && copy_lib_to_test_project; } \
	|| { echo -e "${RED}Building failure${NOCOLOR}"; false; }
}

run_test_project() {
	echo -e "Running..."
	cd $TEST_PROJECT_PATH
	echo 4 | source ./build.sh
	cd -
}

build_and_run() {
	{ build && run_test_project; } || echo -e "${RED}Build&Run failed${NOCOLOR}"
}

clean_all() {
	echo -e "Cleaning..."
	rm -rf $CMAKE_BUILD_DIR/{*,.*} &> /dev/null
	echo -e "${GREEN}All clean!${NOCOLOR}"
}

change_build_type() {
	echo -e "\nBuild type -> ${GREEN}${BUILD_TYPE}${NOCOLOR}"
	echo -e "Choose build type:\n (${RED}d${NOCOLOR})Debug, (${RED}r${NOCOLOR})Release"
	read -n 1 -s input
	case $input in
		d)
			BUILD_TYPE=Debug
			POSTFIX='-d'
			;;
		r)
			BUILD_TYPE=Release
			POSTFIX=''
			;;
		*)
			;;
	esac
}

##### Script Start #####

determine_lib_extension

while true
do
echo -e "\n \
Build type -> ${GREEN}${BUILD_TYPE}${NOCOLOR}\n \
(${RED}1${NOCOLOR}) configure cmake, \n \
(${RED}2${NOCOLOR}) build, \n \
(${RED}3${NOCOLOR}) build & run, \n \
(${RED}4${NOCOLOR}) run test project, \n \
(${RED}5${NOCOLOR}) clean all, \n \
(${RED}6${NOCOLOR}) build test project, \n \
(${RED}b${NOCOLOR}) change build type, \n \
(${GREEN}q${NOCOLOR}) exit\
"

read -n 1 -s input
case $input in
	1)
		configure
		;;
	2)
		build
		;;
	3)
		build_and_run
		;;
	4)
		run_test_project
		;;
	5)
		clean_all
		;;
	6)
		build_test_project
		;;
	b)
		change_build_type
		;;
	*)
		exit
		;;
esac
done
"#;