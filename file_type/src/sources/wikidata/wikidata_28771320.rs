use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28771320: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_320,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office File List",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
