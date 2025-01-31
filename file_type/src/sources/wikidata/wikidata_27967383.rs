use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967383: FileFormat = FileFormat {
    id: 27_967_383,
    puid: "wikidata/27967383",
    name: "RIFF MIDI",
    extensions: &["rmi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
