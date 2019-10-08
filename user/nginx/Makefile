# nginx 的安装
# nginx 的版本
nginx_version := 1.14.2
# nginx 的安装包文件名
nginx_tarball := nginx-$(nginx_version).tar.gz
# nginx 的安装包路径
nginx_tarball_path := src/nginx-$(nginx_version).tar.gz
# 构建目标目录
build_dir := build/$(arch) # arch 是目标机器的类型
# 更具体的构建目标路径
nginx_dir := $(build_dir)/nginx-$(nginx_version)
# 前缀
prefix := $(arch)-linux-musl- # musl 是一个c标准库
# unstripped bin 的构建路径
bin_unstripped := build/$(arch)/nginx_unstripped # unstripped 指的是含有 debug 信息
# stripped bin 的构建路径
bin := build/$(arch)/nginx

# ifeq endif 是 makefile 的控制流程序
ifeq ($(arch), mipsel)
	# := 展开仅发生一次，而 = 展开每次都会发生
	prefix := mipsel-linux-musln32-
endif

# cc 指 c compiler, 需要使用带前缀的 gcc
cc := $(prefix)gcc
# strip 也必须是带前缀的 strip
strip := $(prefix)strip

# $() 是调用宏展开的语法
$(nginx_tarball_path): # 引用 nginx_tarball_path 变量
	# 从 Linux 官网下载 nginx 安装包
	wget https://nginx.org/download/$(nginx_tarball) -O $(nginx_tarball_path)

$(nginx_dir): $(nginx_tarball_path)
	mkdir -p $(build_dir)
	cd $(build_dir) && tar xvf ../../$(nginx_tarball_path)

$(nginx_dir)/objs/nginx: $(nginx_dir)
	sed -i 's/-Wl,-E//' $(nginx_dir)/auto/cc/conf
	# cross compile, do not run
	sed -i 's/ngx_feature_run=yes/ngx_feature_run=no/' $(nginx_dir)/auto/cc/name
	sed -i 's/ngx_test="$$CC /ngx_test="gcc /' $(nginx_dir)/auto/types/sizeof
	cd $(nginx_dir) && ./configure --with-cc=$(cc) --with-cc-opt=-static --with-ld-opt=-static --without-pcre --without-http_rewrite_module --without-http_gzip_module --with-poll_module --without-http_upstream_zone_module
	cd $(nginx_dir) && printf "#ifndef NGX_SYS_NERR\n#define NGX_SYS_NERR  132\n#endif\n" >> objs/ngx_auto_config.h
	cd $(nginx_dir) && printf "#ifndef NGX_HAVE_SVSVSHM\n#define NGX_HAVE_SYSVSHM  1\n#endif\n" >> objs/ngx_auto_config.h
	# FIXME: overflow will occur on 32-bit platforms, this is only a temporary workaround
	cd $(nginx_dir) && make CFLAGS="${CFLGAS} -Wno-error=overflow"

$(bin_unstripped): $(nginx_dir)/objs/nginx
	cp $(nginx_dir)/objs/nginx $(bin_unstripped)

$(bin): $(bin_unstripped)
	cp $(bin_unstripped) $(bin)
	$(strip) $(bin)

.PHONY: all clean

all: $(bin)

clean:
	rm -rf build/$(arch)
