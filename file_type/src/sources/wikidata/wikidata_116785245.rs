use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116785245: FileFormat = FileFormat {
    id: 116_785_245,
    source_type: SourceType::Wikidata,
    name: "602Pro PC Suite macro",
    extensions: &["cnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
