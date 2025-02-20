use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960055: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_055,
        source_type: SourceType::Wikidata,
        name: "Audible Audiobook",
        extensions: &["aa"],
        media_types: &["audio/vnd.audible", "audio/x-pn-audibleaudio"],
        signatures: &[],
        related_formats: &[],
    },
};
