use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58632513: FileFormat = FileFormat {
    id: 58_632_513,
    puid: "wikidata/58632513",
    name: "Corel R.A.V.E.",
    extensions: &["clk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
