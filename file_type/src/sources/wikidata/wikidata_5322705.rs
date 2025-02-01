use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5322705: FileFormat = FileFormat {
    id: 5_322_705,
    puid: "wikidata/5322705",
    name: "Extended Common Object File Format",
    extensions: &["o", "so"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
