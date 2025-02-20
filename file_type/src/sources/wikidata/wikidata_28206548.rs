use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206548: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_548,
        source_type: SourceType::Wikidata,
        name: "MAKIchan Graphics MAX",
        extensions: &["max"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
