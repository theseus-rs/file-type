use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130057181: FileType = FileType {
    file_format: &FileFormat {
        id: 130_057_181,
        source_type: SourceType::Wikidata,
        name: "K source code file",
        extensions: &["k"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
