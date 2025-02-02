use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25101636: FileFormat = FileFormat {
    id: 25_101_636,
    source_type: SourceType::Wikidata,
    name: "IVUE",
    extensions: &["ivue"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
