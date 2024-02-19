; ModuleID = 'probe9.501fd018-cgu.0'
source_filename = "probe9.501fd018-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-n32-S128"
target triple = "riscv32"

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17ha4fff51937d1adf5E"(ptr sret([8 x i8]) %0, double %self) unnamed_addr #0 {
start:
  %_3 = alloca double, align 8
  store double %self, ptr %_3, align 8
  %rt = load double, ptr %_3, align 8, !noundef !0
  %self1 = bitcast double %rt to i64
  store i64 %self1, ptr %0, align 1
  ret void
}

; probe9::probe
; Function Attrs: nounwind
define dso_local void @_ZN6probe95probe17hef8944a5da039387E() unnamed_addr #1 {
start:
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  call void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17ha4fff51937d1adf5E"(ptr sret([8 x i8]) %_1, double 3.140000e+00) #2
  ret void
}

attributes #0 = { inlinehint nounwind "target-cpu"="generic-rv32" "target-features"="+m" }
attributes #1 = { nounwind "target-cpu"="generic-rv32" "target-features"="+m" }
attributes #2 = { nounwind }

!0 = !{}
