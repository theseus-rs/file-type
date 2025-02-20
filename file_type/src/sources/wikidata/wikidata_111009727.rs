use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009727: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_727,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Envelope File format",
        extensions: &["env"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
