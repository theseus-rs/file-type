use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58632423: FileFormat = FileFormat {
    id: 58_632_423,
    puid: "wikidata/58632423",
    name: "Corel R.A.V.E.",
    extensions: &["clk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
