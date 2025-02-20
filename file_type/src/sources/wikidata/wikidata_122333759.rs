use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122333759: FileType = FileType {
    file_format: &FileFormat {
        id: 122_333_759,
        source_type: SourceType::Wikidata,
        name: "Logo Design Studio File",
        extensions: &["lds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
