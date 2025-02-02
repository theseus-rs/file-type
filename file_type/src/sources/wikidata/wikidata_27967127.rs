use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967127: FileFormat = FileFormat {
    id: 27_967_127,
    source_type: SourceType::Wikidata,
    name: "CMS",
    extensions: &["cms"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
