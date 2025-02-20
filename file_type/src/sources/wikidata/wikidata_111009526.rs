use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009526: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_526,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Certificate File format",
        extensions: &["cer"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
