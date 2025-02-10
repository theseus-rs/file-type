use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_99844768: FileType = FileType {
    file_format: &FileFormat {
        id: 99_844_768,
        source_type: SourceType::Wikidata,
        name: "MicroStation Base File",
        extensions: &["bse"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
