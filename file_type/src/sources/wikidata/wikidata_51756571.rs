use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51756571: FileType = FileType {
    file_format: &FileFormat {
        id: 51_756_571,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Slide Library",
        extensions: &["slb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
