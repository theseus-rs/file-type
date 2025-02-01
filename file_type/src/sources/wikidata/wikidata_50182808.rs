use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50182808: FileFormat = FileFormat {
    id: 50_182_808,
    puid: "wikidata/50182808",
    name: "Open Inventor File Format, v2",
    extensions: &["iv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
