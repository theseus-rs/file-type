use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135395124: FileType = FileType {
    file_format: &FileFormat {
        id: 135_395_124,
        source_type: SourceType::Wikidata,
        name: "Not Quite C file format",
        extensions: &["nqc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
