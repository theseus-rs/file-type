use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205879: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_879,
        source_type: SourceType::Wikidata,
        name: "CUT",
        extensions: &["cut"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
