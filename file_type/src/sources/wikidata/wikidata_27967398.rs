use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967398: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_398,
        source_type: SourceType::Wikidata,
        name: "AdLib Visual Composer / Roland Synthesizer song",
        extensions: &["rol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
