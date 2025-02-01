use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116869035: FileFormat = FileFormat {
    id: 116_869_035,
    puid: "wikidata/116869035",
    name: "Summitsoft Business Card",
    extensions: &["crd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
