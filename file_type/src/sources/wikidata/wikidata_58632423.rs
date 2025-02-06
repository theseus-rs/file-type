use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58632423: FileFormat = FileFormat {
    id: 58_632_423,
    source_type: SourceType::Wikidata,
    name: "Corel R.A.V.E.",
    extensions: &["clk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
