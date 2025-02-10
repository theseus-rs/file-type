use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28771302: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_302,
        source_type: SourceType::Wikidata,
        name: "Matlab figure",
        extensions: &["fig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
