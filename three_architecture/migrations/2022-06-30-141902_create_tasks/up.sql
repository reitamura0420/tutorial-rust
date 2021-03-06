-- Your SQL goes here
CREATE TABLE tasks (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  name TEXT NOT NULL,
  user_id INTEGER NOT NULL,
  term DATETIME NOT NULL,
  update_term_count INTEGER NOT NULL DEFAULT 0,
  is_completed BOOLEAN NOT NULL DEFAULT 0,
  FOREIGN KEY (user_id)
  REFERENCES users(id) ON DELETE CASCADE
) DEFAULT CHARACTER SET= utf8mb4;