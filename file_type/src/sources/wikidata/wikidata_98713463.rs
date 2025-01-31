use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_98713463: FileFormat = FileFormat {
    id: 98_713_463,
    puid: "wikidata/98713463",
    name: "POV-Ray",
    extensions: &["pov"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
