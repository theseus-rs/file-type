use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122583982: FileType = FileType {
    file_format: &FileFormat {
        id: 122_583_982,
        source_type: SourceType::Wikidata,
        name: "Zetafax Fax Image File (Normal)",
        extensions: &["g3n"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
