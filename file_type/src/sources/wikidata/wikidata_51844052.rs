use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51844052: FileFormat = FileFormat {
    id: 51_844_052,
    puid: "wikidata/51844052",
    name: "Microsoft Visual Modeller Petal file (ASCII)",
    extensions: &["ptl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
