use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59820771: FileFormat = FileFormat {
    id: 59_820_771,
    puid: "wikidata/59820771",
    name: "Corel R.A.V.E.",
    extensions: &["clk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
