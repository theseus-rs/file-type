use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129922769: FileFormat = FileFormat {
    id: 129_922_769,
    puid: "wikidata/129922769",
    name: "Jasmin file format",
    extensions: &["j"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
