use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7903498: FileFormat = FileFormat {
    id: 7_903_498,
    source_type: SourceType::Wikidata,
    name: "UTZ",
    extensions: &["utz"],
    media_types: &["application/vnd.uiq.theme"],
    internal_signatures: &[],
    related_formats: &[],
};
