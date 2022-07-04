use crate::presentation::presentation_tasks::*;

pub fn register_tasks(task: RequestTask) -> bool {
    // ユーザーを取得する
    // 「活性」ステータスのユーザーのみ担当に設定できる
    // DBリポジトリを呼ぶ
    crate::data_access::data_access_tasks::post_tasks(task);
    // Resultの分岐処理を行う
    true
}
