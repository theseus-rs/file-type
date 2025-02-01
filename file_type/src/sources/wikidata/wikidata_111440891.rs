use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440891: FileFormat = FileFormat {
    id: 111_440_891,
    puid: "wikidata/111440891",
    name: "VoiceXML File",
    extensions: &["vxml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
