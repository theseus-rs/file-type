use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128597078: FileType = FileType {
    file_format: &FileFormat {
        id: 128_597_078,
        source_type: SourceType::Wikidata,
        name: "AmbientTalk file",
        extensions: &["at"],
        media_types: &["text/x-ambienttalk"],
        signatures: &[],
        related_formats: &[],
    },
};
