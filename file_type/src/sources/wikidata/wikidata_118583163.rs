use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118583163: FileFormat = FileFormat {
    id: 118_583_163,
    puid: "wikidata/118583163",
    name: "Kinetic Project",
    extensions: &["kin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
