cmake_minimum_required(VERSION 3.31.5)
project(Neko C)

# 指定 Chibi 的头文件和库路径
include_directories(/home/MinkieYume/.local/include)
include_directories(/usr/include)

# 寻找库并将库的路径保存到CHIBI_LIB变量中
find_library(CHIBI_LIB chibi-scheme PATHS /home/MinkieYume/.local/lib)
find_library(NNG_LIB nng PATHS /usr/lib64)

# 添加源文件
add_executable(${PROJECT_NAME} src/main.c)

# 对可执行文件链接库
target_link_libraries(${PROJECT_NAME} ${CHIBI_LIB} ${NNG_LIB})
