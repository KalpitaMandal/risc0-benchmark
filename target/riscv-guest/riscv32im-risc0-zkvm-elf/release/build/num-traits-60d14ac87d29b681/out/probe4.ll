; ModuleID = 'probe4.994b0798-cgu.0'
source_filename = "probe4.994b0798-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-n32-S128"
target triple = "riscv32"

@alloc_e5fa6b9ac875ecba95591a733b51e2fb = private unnamed_addr constant <{ [68 x i8] }> <{ [68 x i8] c"/opt/actions-runner/_work/rust/rust/rust/library/core/src/num/mod.rs" }>, align 1
@alloc_2cf15c57c01db7956a8a43607f5f7608 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_e5fa6b9ac875ecba95591a733b51e2fb, [12 x i8] c"D\00\00\00/\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nounwind
define dso_local void @_ZN6probe45probe17h8b15e93a75098ffdE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7cfd4caa35e6e370E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9700954fa59d8520E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_2cf15c57c01db7956a8a43607f5f7608) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7cfd4caa35e6e370E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h9700954fa59d8520E(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic-rv32" "target-features"="+m" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic-rv32" "target-features"="+m" }
attributes #3 = { noreturn nounwind }
