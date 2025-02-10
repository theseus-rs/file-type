use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009741: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_741,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Fax Cover File format",
        extensions: &["fax"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
