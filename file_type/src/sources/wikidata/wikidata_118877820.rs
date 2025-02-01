use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118877820: FileFormat = FileFormat {
    id: 118_877_820,
    puid: "wikidata/118877820",
    name: "Open Scripting Architecture binary script",
    extensions: &["scpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
