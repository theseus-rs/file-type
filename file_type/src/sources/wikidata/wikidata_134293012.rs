use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134293012: FileType = FileType {
    file_format: &FileFormat {
        id: 134_293_012,
        source_type: SourceType::Wikidata,
        name: "Clipper overlay file",
        extensions: &["ovl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
