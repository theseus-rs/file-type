use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959791: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_791,
        source_type: SourceType::Wikidata,
        name: "Ableton Device Group",
        extensions: &["adg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
