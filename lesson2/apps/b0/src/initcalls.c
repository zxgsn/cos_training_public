/* Helper for getting init_calls_[start/end] */

/*
.text : ALIGN(4K) {
        stext = .;
        *(.text.boot)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
        init_calls_start = .;
        KEEP(*(.init_calls))
        init_calls_end = .;
    }
*/

// 本来想直接使用上面init_calls_start的符号， 其实要使用init_calls_start[]才行;
// 直接使用init_calls_end 会发现值为0， 由于所有名为 .init_calls 的段都被合并到了最终的可执行文件中，
// 所以最终的可执行文件的 .init_calls 段结束位置为 0，
// 即所有 .init_calls 段的总大小为 0。因此，在链接脚本中，init_calls_end 的位置也被设置为 0。

extern void* init_calls_start[];
extern void* init_calls_end[];

#define __READ_ONCE(x) (*(const volatile unsigned long int *) & (x))

unsigned long int initcalls_start() {
    /* Todo: fix it! */
    unsigned long int address =  (unsigned long int)init_calls_start;
    return __READ_ONCE(address);
}

unsigned long int initcalls_end() {
    /* Todo: fix it! */
    unsigned long int address =  (unsigned long int)init_calls_end;
    return __READ_ONCE(address);
}
