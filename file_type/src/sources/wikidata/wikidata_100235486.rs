use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100235486: FileType = FileType {
    file_format: &FileFormat {
        id: 100_235_486,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 2018/2019/2020/2021/2022/2023",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
