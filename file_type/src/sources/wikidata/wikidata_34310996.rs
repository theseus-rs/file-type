use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34310996: FileFormat = FileFormat {
    id: 34_310_996,
    source_type: SourceType::Wikidata,
    name: "Security Catalog",
    extensions: &["cat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
