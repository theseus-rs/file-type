use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206325: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_325,
        source_type: SourceType::Wikidata,
        name: "Img Software Set Image Attributes",
        extensions: &["a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
