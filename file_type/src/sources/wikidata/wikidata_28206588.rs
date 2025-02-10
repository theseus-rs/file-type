use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206588: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_588,
        source_type: SourceType::Wikidata,
        name: "Microsoft Image Composer file",
        extensions: &["mic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
