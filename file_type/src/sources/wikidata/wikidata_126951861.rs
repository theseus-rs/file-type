use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126951861: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_861,
        source_type: SourceType::Wikidata,
        name: "Scala source code file",
        extensions: &["scala"],
        media_types: &["text/x-scala"],
        signatures: &[],
        related_formats: &[],
    },
};
