project(codal-steami-samples)

include("${CODAL_UTILS_LOCATION}")

# find sources and headers
RECURSIVE_FIND_FILE(TOP_LEVEL_INCLUDE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/source" "*.h")
RECURSIVE_FIND_FILE(TOP_LEVEL_SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/source" "*.c")
RECURSIVE_FIND_FILE(TOP_LEVEL_SOURCE_FILES0 "${CMAKE_CURRENT_SOURCE_DIR}/source" "*.cpp")

RECURSIVE_FIND_DIR(SAMPLES_INCLUDE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/samples" "*.h")
RECURSIVE_FIND_FILE(SAMPLES_SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/samples" "*.c")
RECURSIVE_FIND_FILE(SAMPLES_SOURCE_FILES0 "${CMAKE_CURRENT_SOURCE_DIR}/samples" "*.cpp")

string(TOLOWER "${CODAL_TARGET_PROCESSOR}" TARGET_PROCESSOR)

# add them
include_directories(${INCLUDE_DIRS} ${TOP_LEVEL_INCLUDE_DIRS} ${SAMPLES_INCLUDE_DIRS})

# create our target
add_library(codal-steami-samples ${TOP_LEVEL_SOURCE_FILES} ${TOP_LEVEL_SOURCE_FILES0} ${SAMPLES_SOURCE_FILES} ${SAMPLES_SOURCE_FILES0})

target_link_libraries(
    codal-steami-samples
    codal-stm32-STEAM32_WB55RG
    codal-stm32
    codal-core
)

# expose it to parent cmake.
target_include_directories(codal-steami-samples PUBLIC ${TOP_LEVEL_INCLUDE_DIRS} ${SAMPLES_INCLUDE_DIRS})
