use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34310996: FileFormat = FileFormat {
    id: 34_310_996,
    source_type: SourceType::Wikidata,
    name: "Security Catalog",
    extensions: &["cat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
