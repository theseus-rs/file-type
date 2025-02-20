use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47539043: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_043,
        source_type: SourceType::Wikidata,
        name: "AutoCAD dbConnect Query Set",
        extensions: &["dbq"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
