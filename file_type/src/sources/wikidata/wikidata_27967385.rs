use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967385: FileFormat = FileFormat {
    id: 27_967_385,
    puid: "wikidata/27967385",
    name: "Extended MIDI",
    extensions: &["xmi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
