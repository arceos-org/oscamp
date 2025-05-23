#!/bin/bash

cd arceos || {
    echo "Failed to change directory to arceos"
    exit 1
}

search_texts() {
    local texts=("$@")

    for text in "${texts[@]}"; do
        if ! grep -q -F "$text" tmpfile; then
            return 1
        fi
    done

    return 0
}
run_test() {

    local args=("$@")

    local app=$1
    local blk=$2
    local payload=$3
    local update_disk_file=$4
    local eval_code=$5

    local tmp_file=tmpfile

    rm -rf disk.img pflash.img $tmp_file

    make disk_img >/dev/null 2>&1
    make pflash_img >/dev/null 2>&1

    if [[ -n $payload ]]; then
        make payload >/dev/null 2>&1
    fi

    if [[ -n $eval_code ]]; then
        eval "$eval_code" >/dev/null 2>&1
    fi

    if [[ -n $update_disk_file ]]; then
        ./update_disk.sh "$update_disk_file" >/dev/null 2>&1
    fi

    make run A="$app" BLK="$blk" >"$tmp_file" 2>&1
    local OUTPUT=$?

    local SEARCH=0
    if [ "${#args[@]}" -gt 5 ]; then
        if ! search_texts "${args[@]:5}"; then
            SEARCH=1
        fi
    fi

    if [ "$OUTPUT" -eq 0 ] && [ "$SEARCH" -eq 0 ]; then
        echo "$app" "Success" >>Success.log
    else
        echo $OUTPUT $SEARCH
        echo "$app" "Error" >>Error.log
    fi

}

rm -rf Success.log Error.log

# app路径, 是否挂载磁盘(n=否/y=是), 是否重新编译payload, 是否更新磁盘文件, 是否在更新磁盘文件前有额外操作,  之后所有传入的都是需要被查询的参数
run_test "tour/u_1_0" "n" "" "" "" "Hello, Arceos!"
run_test "tour/u_2_0" "n" "" "" "" "Alloc Vec" "Alloc String:"
run_test "tour/u_3_0" "n" "" "" "" "Try to access dev region" "Got pflash magic: pfld"
run_test "tour/u_4_0" "n" "" "" "" "Got pflash magic: pfld" "Multi-task OK!" "Spawned-thread ..."
run_test "tour/u_5_0" "n" "" "" "" "worker2 ok!" "worker1 ok!" "Multi-task OK!"
run_test "tour/u_6_0" "n" "" "" "" "Multi-task(Preemptible) ok!"
run_test "tour/u_6_1" "n" "" "" "" "worker2 ok!" "worker1 ok!" "WaitQ ok!"
run_test "tour/u_7_0" "y" "" "" "" "[mkfs.fat]" "worker1 ok!" "Load app from disk ok!"
run_test "tour/u_8_0" "y" "" "" "" "worker1 checks code:" "worker1 ok!" "Load app from disk ok!"
run_test "tour/m_1_0" "y" "y" "payload/origin/origin" "" "monolithic kernel exit [Some(0)] normally!"
run_test "tour/m_1_1" "y" "y" "payload/origin/origin" "" "monolithic kernel exit [Some(0)] normally!"
run_test "tour/m_2_0" "y" "y" "payload/origin/origin" "" "handle page fault OK!" "monolithic kernel exit [Some(0)] normally!"
run_test "tour/m_3_0" "y" "y" "payload/hello_c/hello" "" "Hello, UserApp!" "monolithic kernel exit [Some(0)] normally!"
run_test "tour/m_3_1" "y" "y" "payload/fileops_c/fileops" "" "FileOps ok!" "monolithic kernel exit [Some(0)] normally!"
run_test "tour/h_1_0" "y" "y" "payload/skernel/skernel" "" "Shutdown vm normally!" "Hypervisor ok!"
run_test "tour/h_2_0" "y" "y" "tour/u_3_0/u_3_0_riscv64-qemu-virt.bin" "make A=tour/u_3_0/" "Got pflash magic: pfld"
run_test "tour/h_3_0" "y" "y" "tour/u_6_0/u_6_0_riscv64-qemu-virt.bin" "make A=tour/u_6_0/" "Multi-task(Preemptible) ok!"
run_test "tour/h_4_0" "y" "y" "tour/m_1_1/m_1_1_riscv64-qemu-virt.bin" "make A=tour/m_1_1" "monolithic kernel exit [Some(0)] normally!" "handle_syscall ..."

if [[ -s Error.log ]]; then
    cat Error.log # Print the content of Error.log
    rm -f Error.log Suceess.log 
    exit 1        # Exit with an error code to signal failure in CI
else
    echo "All Success"
    rm -f Error.log Suceess.log
    exit 0
fi