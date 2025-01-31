use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128628799: FileFormat = FileFormat {
    id: 128_628_799,
    puid: "wikidata/128628799",
    name: "BARE schema source",
    extensions: &["bare"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
