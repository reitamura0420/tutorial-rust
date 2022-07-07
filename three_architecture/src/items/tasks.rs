use crate::presentation::tasks::*;

pub fn register_tasks(task: RequestTask) -> bool {
    // ユーザーを取得する
    // 「活性」ステータスのユーザーのみ担当に設定できる
    // DBリポジトリを呼ぶ
    crate::data_access::tasks::insert_tasks(task);
    true
}

pub fn update_schedules(id: i32) -> bool {
    crate::data_access::tasks::update_schedules(id);
    true
}
