use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123679549: FileType = FileType {
    file_format: &FileFormat {
        id: 123_679_549,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing 2021",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
