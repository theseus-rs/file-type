use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58335773: FileFormat = FileFormat {
    id: 58_335_773,
    puid: "wikidata/58335773",
    name: "Verity Collection Stop List",
    extensions: &["stp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
