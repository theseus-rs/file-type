use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960055: FileFormat = FileFormat {
    id: 27_960_055,
    puid: "wikidata/27960055",
    name: "Audible Audiobook",
    extensions: &["aa", "aa"],
    media_types: &["audio/vnd.audible", "audio/x-pn-audibleaudio"],
    internal_signatures: &[],
    related_formats: &[],
};
