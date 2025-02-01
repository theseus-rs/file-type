use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131304008: FileFormat = FileFormat {
    id: 131_304_008,
    puid: "wikidata/131304008",
    name: "Riverbed Stingray Traffic Manager file format",
    extensions: &["rts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
