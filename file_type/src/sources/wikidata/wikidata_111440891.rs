use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111440891: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_891,
        source_type: SourceType::Wikidata,
        name: "VoiceXML File",
        extensions: &["vxml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
