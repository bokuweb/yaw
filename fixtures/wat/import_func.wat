(module
  (func $i (import "imports" "imported_func") (param i32) (result i32))
  (func (export "exported_func") (result i32)
    i32.const 42
    call $i))

;; const importObject = {
;;   imports: {
;;       imported_func: function(arg) {
;;         return 3;
;;       }
;;   }
;; };