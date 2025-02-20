use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34305583: FileType = FileType {
    file_format: &FileFormat {
        id: 34_305_583,
        source_type: SourceType::Wikidata,
        name: "Racket script",
        extensions: &["rkt", "rktd", "rktl"],
        media_types: &["application/x-racket", "text/x-racket"],
        signatures: &[],
        related_formats: &[],
    },
};
