use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206328: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_328,
        source_type: SourceType::Wikidata,
        name: "Img Software Set Red Component",
        extensions: &["r"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
