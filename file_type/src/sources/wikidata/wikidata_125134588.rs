use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125134588: FileFormat = FileFormat {
    id: 125_134_588,
    puid: "wikidata/125134588",
    name: "YAM Unique ID Listing",
    extensions: &["uidl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
