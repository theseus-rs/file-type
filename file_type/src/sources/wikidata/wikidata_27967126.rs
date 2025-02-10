use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967126: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_126,
        source_type: SourceType::Wikidata,
        name: "CMR",
        extensions: &["cmr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
