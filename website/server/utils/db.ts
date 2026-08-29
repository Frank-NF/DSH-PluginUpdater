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

    CREATE TABLE IF NOT EXISTS feedback (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      plugin_id TEXT NOT NULL,
      plugin_name TEXT NOT NULL DEFAULT '',
      type TEXT NOT NULL DEFAULT 'suggestion' CHECK(type IN ('bug','suggestion','experience','question','other')),
      content TEXT NOT NULL CHECK(length(content) <= 2000),
      contact TEXT,
      user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
      ip_hash TEXT,
      status TEXT NOT NULL DEFAULT 'open' CHECK(status IN ('open','processing','resolved','closed')),
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      updated_at TEXT NOT NULL DEFAULT (datetime('now'))
    );

    CREATE TABLE IF NOT EXISTS plugin_compat (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      plugin_id TEXT NOT NULL,
      dsh_version TEXT NOT NULL DEFAULT '*',
      compatible INTEGER NOT NULL DEFAULT 1,
      note TEXT,
      updated_at TEXT NOT NULL DEFAULT (datetime('now')),
      UNIQUE(plugin_id, dsh_version)
    );

    CREATE TABLE IF NOT EXISTS plugin_conflicts (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      plugin_id TEXT NOT NULL,
      conflict_with TEXT NOT NULL,
      reason TEXT,
      severity TEXT NOT NULL DEFAULT 'warn' CHECK(severity IN ('warn','block')),
      updated_at TEXT NOT NULL DEFAULT (datetime('now')),
      UNIQUE(plugin_id, conflict_with)
    );

    CREATE INDEX IF NOT EXISTS idx_favorites_user ON favorites(user_id);
    CREATE INDEX IF NOT EXISTS idx_favorites_plugin ON favorites(plugin_id);
    CREATE INDEX IF NOT EXISTS idx_comments_plugin ON comments(plugin_id, deleted);
    CREATE INDEX IF NOT EXISTS idx_shares_plugin ON shares(plugin_id);
    CREATE INDEX IF NOT EXISTS idx_feedback_plugin ON feedback(plugin_id, status);
    CREATE INDEX IF NOT EXISTS idx_feedback_user ON feedback(user_id);
    CREATE INDEX IF NOT EXISTS idx_compat_plugin ON plugin_compat(plugin_id);
    CREATE INDEX IF NOT EXISTS idx_conflicts_plugin ON plugin_conflicts(plugin_id);
  `)
}
