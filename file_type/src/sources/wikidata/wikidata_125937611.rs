use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125937611: FileFormat = FileFormat {
    id: 125_937_611,
    puid: "wikidata/125937611",
    name: "Enigma Binary File 1",
    extensions: &["mus"],
    media_types: &["application/vnd.makemusic.notation"],
    internal_signatures: &[],
    related_formats: &[],
};
