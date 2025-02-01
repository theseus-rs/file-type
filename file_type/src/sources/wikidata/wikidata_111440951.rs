use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440951: FileFormat = FileFormat {
    id: 111_440_951,
    puid: "wikidata/111440951",
    name: "BASIC Source Code File",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
