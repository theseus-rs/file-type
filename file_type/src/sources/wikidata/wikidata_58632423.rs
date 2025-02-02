use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58632423: FileFormat = FileFormat {
    id: 58_632_423,
    source_type: SourceType::Wikidata,
    name: "Corel R.A.V.E.",
    extensions: &["clk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
