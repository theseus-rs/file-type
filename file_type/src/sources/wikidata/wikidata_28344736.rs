use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28344736: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_736,
        source_type: SourceType::Wikidata,
        name: "Macintosh resource file",
        extensions: &["dfont", "rsrc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
