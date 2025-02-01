use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59999470: FileFormat = FileFormat {
    id: 59_999_470,
    puid: "wikidata/59999470",
    name: "ESRI Spatial Index File",
    extensions: &["sbn", "sbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
