/**
 * SQLite 数据库层
 * 数据文件: .data/dsh-website.db（自动创建目录）
 * 表: users / favorites / comments / shares
 */
import Database from 'better-sqlite3'
import { existsSync, mkdirSync } from 'node:fs'
import { join } from 'node:path'

let _db: Database.Database | null = null

export function getDB(): Database.Database {
  if (_db) return _db

  const dataDir = join(process.cwd(), '..', '..', '.data')
  if (!existsSync(dataDir)) {
    mkdirSync(dataDir, { recursive: true })
  }

  _db = new Database(join(dataDir, 'dsh-website.db'))
  _db.pragma('journal_mode = WAL')
  _db.pragma('foreign_keys = ON')

  migrate(_db)
  return _db
}

function migrate(db: Database.Database) {
  db.exec(`
    CREATE TABLE IF NOT EXISTS users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      email TEXT UNIQUE,
      password_hash TEXT,
      github_id TEXT UNIQUE,
      github_login TEXT,
      display_name TEXT NOT NULL,
      avatar_url TEXT,
      role TEXT NOT NULL DEFAULT 'user',
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      last_login_at TEXT
    );

    CREATE TABLE IF NOT EXISTS favorites (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      plugin_id TEXT NOT NULL,
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      UNIQUE(user_id, plugin_id)
    );

    CREATE TABLE IF NOT EXISTS comments (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      plugin_id TEXT NOT NULL,
      content TEXT NOT NULL CHECK(length(content) <= 1000),
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      deleted INTEGER NOT NULL DEFAULT 0
    );

    CREATE TABLE IF NOT EXISTS shares (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
      plugin_id TEXT NOT NULL,
      channel TEXT,
      created_at TEXT NOT NULL DEFAULT (datetime('now'))
    );

    CREATE INDEX IF NOT EXISTS idx_favorites_user ON favorites(user_id);
    CREATE INDEX IF NOT EXISTS idx_favorites_plugin ON favorites(plugin_id);
    CREATE INDEX IF NOT EXISTS idx_comments_plugin ON comments(plugin_id, deleted);
    CREATE INDEX IF NOT EXISTS idx_shares_plugin ON shares(plugin_id);
  `)
}
