use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134274041: FileType = FileType {
    file_format: &FileFormat {
        id: 134_274_041,
        source_type: SourceType::Wikidata,
        name: "Clipper object file",
        extensions: &["obj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
