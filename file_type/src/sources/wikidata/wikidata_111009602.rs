use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009602: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_602,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Business Card File format",
        extensions: &["biz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
