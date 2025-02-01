use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125947385: FileFormat = FileFormat {
    id: 125_947_385,
    puid: "wikidata/125947385",
    name: "Finale Notation File 2014+",
    extensions: &["musx"],
    media_types: &["application/vnd.makemusic.notation"],
    internal_signatures: &[],
    related_formats: &[],
};
