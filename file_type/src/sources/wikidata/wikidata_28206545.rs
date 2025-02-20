use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206545: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_545,
        source_type: SourceType::Wikidata,
        name: "MAKIchan Graphics MAG",
        extensions: &["mag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
