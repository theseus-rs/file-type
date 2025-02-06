use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960055: FileFormat = FileFormat {
    id: 27_960_055,
    source_type: SourceType::Wikidata,
    name: "Audible Audiobook",
    extensions: &["aa"],
    media_types: &["audio/vnd.audible", "audio/x-pn-audibleaudio"],
    signatures: &[],
    related_formats: &[],
};
