use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111263105: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_105,
        source_type: SourceType::Wikidata,
        name: "Typhoon wave file",
        extensions: &["c01"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
