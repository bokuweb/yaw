
(module
  (import "env" "global1" (global i32))
  (func $global (param $lhs i32) (param $rhs i32) (result i32)
    get_global 0)
  (export "global" (func $global))
)


;; const global1 = new WebAssembly.Global({value: "i32", mutable: false}, 42);
;; 
;; fetch('../out/main.wasm').then(response =>
;;   response.arrayBuffer()
;; ).then(bytes => WebAssembly.instantiate(bytes, { env: { global1 }})).then(results => {
;;   instance = results.instance;
;;   document.getElementById("container").textContent = instance.exports.global(1,1);
;; }).catch(console.error);

