use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28770260: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_260,
        source_type: SourceType::Wikidata,
        name: "Hugo",
        extensions: &["hex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
