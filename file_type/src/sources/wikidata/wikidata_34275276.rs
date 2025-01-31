use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34275276: FileFormat = FileFormat {
    id: 34_275_276,
    puid: "wikidata/34275276",
    name: "Numbers Zipped",
    extensions: &["numbers.zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
