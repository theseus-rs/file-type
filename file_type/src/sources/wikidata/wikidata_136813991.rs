use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136813991: FileType = FileType {
    file_format: &FileFormat {
        id: 136_813_991,
        source_type: SourceType::Wikidata,
        name: "Pro/ENGINEER Assembly file",
        extensions: &["asm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
