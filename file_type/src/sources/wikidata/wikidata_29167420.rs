use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167420: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_420,
        source_type: SourceType::Wikidata,
        name: "Internet Adventure Game Engine source code",
        extensions: &["ic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
