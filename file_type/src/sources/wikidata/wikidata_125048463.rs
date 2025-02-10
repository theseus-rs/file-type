use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125048463: FileType = FileType {
    file_format: &FileFormat {
        id: 125_048_463,
        source_type: SourceType::Wikidata,
        name: "Yoshimi Scale Settings file",
        extensions: &["xsz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
