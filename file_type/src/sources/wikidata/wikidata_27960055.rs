use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960055: FileFormat = FileFormat {
    id: 27_960_055,
    source_type: SourceType::Wikidata,
    name: "Audible Audiobook",
    extensions: &["aa"],
    media_types: &["audio/vnd.audible", "audio/x-pn-audibleaudio"],
    internal_signatures: &[],
    related_formats: &[],
};
