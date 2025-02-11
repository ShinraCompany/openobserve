// Copyright 2023 Zinc Labs Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub mod disk;
pub mod memory;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CacheType {
    Disk,
    Memory,
    None,
}

pub async fn init() -> Result<(), anyhow::Error> {
    disk::init().await?;
    memory::init().await?;
    Ok(())
}

pub async fn download(session_id: &str, file: &str) -> Result<(), anyhow::Error> {
    if config::CONFIG.memory_cache.enabled {
        memory::download(session_id, file).await
    } else if config::CONFIG.disk_cache.enabled {
        disk::download(session_id, file).await
    } else {
        Ok(())
    }
}
