use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113557073: FileFormat = FileFormat {
    id: 113_557_073,
    puid: "wikidata/113557073",
    name: "CloneCD Image",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
