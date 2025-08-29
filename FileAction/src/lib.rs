use serde::{Serialize, de::DeserializeOwned};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// ファイル操作ユーティリティ構造体
pub struct FileAction;

impl FileAction {
    /// 相対パスを絶対パスに変換する
    pub fn convert_file_link(current_file_path: &str) -> PathBuf {
        let base = std::env::current_exe().unwrap();
        let base_dir = base.parent().unwrap();
        base_dir.join(current_file_path)
    }

    /// テキストファイルの読み込み
    pub fn read(current_file_path: &str) -> std::io::Result<String> {
        let file_path = Self::convert_file_link(current_file_path);
        fs::read_to_string(file_path)
    }

    /// CSVファイルを読み込み2次元配列を返す
    pub fn read_csv(current_file_path: &str) -> std::io::Result<Vec<Vec<String>>> {
        let text_data = Self::read(current_file_path)?;
        let result = text_data
            .lines()
            .map(|line| line.split(',').map(|s| s.to_string()).collect())
            .collect();
        Ok(result)
    }

    /// 型指定可能なJSONファイルの読み込み
    pub fn read_json<T: DeserializeOwned>(current_file_path: &str) -> std::io::Result<T> {
        let file_path = Self::convert_file_link(current_file_path);
        let text = fs::read_to_string(file_path)?;
        let obj = serde_json::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(obj)
    }

    /// テキストファイルに書き込む
    pub fn write(
        current_file_path: &str,
        write_string: &str,
        file_read_mode: &str,
    ) -> std::io::Result<()> {
        let file_path = Self::convert_file_link(current_file_path);
        let mut options = OpenOptions::new();
        options.write(true).create(true);
        match file_read_mode {
            "w" => {
                options.truncate(true);
            }
            "a" => {
                options.append(true);
            }
            _ => {}
        }
        let mut file = options.open(file_path)?;
        file.write_all(write_string.as_bytes())?;
        Ok(())
    }

    /// テキストファイルに追記
    pub fn write_add(current_file_path: &str, write_string: &str) -> std::io::Result<()> {
        Self::write(current_file_path, write_string, "a")
    }

    /// テキストファイルに上書き
    pub fn write_new(current_file_path: &str, write_string: &str) -> std::io::Result<()> {
        Self::write(current_file_path, write_string, "w")
    }

    /// 型指定可能なJSONファイルの書き込み
    pub fn write_json<T: Serialize>(
        current_file_path: &str,
        obj: &T,
        space: Option<usize>,
    ) -> std::io::Result<()> {
        let file_path = Self::convert_file_link(current_file_path);
        let json = match space {
            Some(s) => serde_json::to_string_pretty(obj).map(|j| j.replace("\n", &" ".repeat(s))),
            None => serde_json::to_string(obj),
        }
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(file_path, json)?;
        Ok(())
    }

    /// テキストファイルの中身をクリアする
    pub fn clear(current_file_path: &str) -> std::io::Result<()> {
        Self::write_new(current_file_path, "")
    }
}
