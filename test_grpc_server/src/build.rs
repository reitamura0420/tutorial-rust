// protoファイルをprostによってコンパイルし、サービススタブを生成します。
// NixOSを参考に作られている
// NixOS では、PROTOC と PROTOC_INCLUDE の場所を明示的に指定するのが良いと提唱されている
// なぜなら、もし `prost_build::compile_protos` が結果パッケージの生成に失敗したら、 `include!(concat!(env!("OUT_DIR"), "/resultant.rs"));` が `No such file or directory` エラーで失敗するまで失敗は明らかにはならないからである。

