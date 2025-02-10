use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127691086: FileType = FileType {
    file_format: &FileFormat {
        id: 127_691_086,
        source_type: SourceType::Wikidata,
        name: "Dart file",
        extensions: &["dart"],
        media_types: &["text/x-dart"],
        signatures: &[],
        related_formats: &[],
    },
};
