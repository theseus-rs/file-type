use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118140134: FileType = FileType {
    file_format: &FileFormat {
        id: 118_140_134,
        source_type: SourceType::Wikidata,
        name: "Serenade Project File",
        extensions: &["ssp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
