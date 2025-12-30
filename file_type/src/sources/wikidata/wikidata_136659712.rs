use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136659712: FileType = FileType {
    file_format: &FileFormat {
        id: 136_659_712,
        source_type: SourceType::Wikidata,
        name: "OpenDX file",
        extensions: &["dx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
