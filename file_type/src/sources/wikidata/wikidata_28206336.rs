use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206336: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_336,
        source_type: SourceType::Wikidata,
        name: "Img Software Set Blue Component",
        extensions: &["b"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
