use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59820771: FileFormat = FileFormat {
    id: 59_820_771,
    source_type: SourceType::Wikidata,
    name: "Corel R.A.V.E.",
    extensions: &["clk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
