use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771316: FileFormat = FileFormat {
    id: 28_771_316,
    puid: "wikidata/28771316",
    name: "Micrografx Draw",
    extensions: &["drw"],
    media_types: &["application/x-mgx-designer"],
    internal_signatures: &[],
    related_formats: &[],
};
