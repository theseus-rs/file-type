use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967105: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_105,
        source_type: SourceType::Wikidata,
        name: "SHO",
        extensions: &["sho"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
