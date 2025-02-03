use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5299371: FileFormat = FileFormat {
    id: 5_299_371,
    source_type: SourceType::Wikidata,
    name: "dotXSI",
    extensions: &["xsi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
