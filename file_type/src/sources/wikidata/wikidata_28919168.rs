use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919168: FileFormat = FileFormat {
    id: 28_919_168,
    puid: "wikidata/28919168",
    name: "GHS Part Maker",
    extensions: &["pm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
