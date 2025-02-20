use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47539217: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_217,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Batch Plot File, version 1.0-R14",
        extensions: &["bp2", "bpl"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
