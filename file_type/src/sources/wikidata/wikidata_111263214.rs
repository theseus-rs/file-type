use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111263214: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_214,
        source_type: SourceType::Wikidata,
        name: "3GPP 'AMR interface format 2'",
        extensions: &["cod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
