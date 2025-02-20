use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122583807: FileType = FileType {
    file_format: &FileFormat {
        id: 122_583_807,
        source_type: SourceType::Wikidata,
        name: "Zetafax Fax Image File (Fine)",
        extensions: &["g3f"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
