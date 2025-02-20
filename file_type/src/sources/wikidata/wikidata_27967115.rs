use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967115: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_115,
        source_type: SourceType::Wikidata,
        name: "Art of Noise module",
        extensions: &["aon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
