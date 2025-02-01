use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125938431: FileFormat = FileFormat {
    id: 125_938_431,
    puid: "wikidata/125938431",
    name: "Enigma Binary File 2",
    extensions: &["mus"],
    media_types: &["application/vnd.makemusic.notation"],
    internal_signatures: &[],
    related_formats: &[],
};
