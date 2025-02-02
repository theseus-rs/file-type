use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125149197: FileFormat = FileFormat {
    id: 125_149_197,
    source_type: SourceType::Wikidata,
    name: "Units Data File",
    extensions: &["units"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
