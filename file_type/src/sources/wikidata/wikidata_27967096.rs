use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967096: FileFormat = FileFormat {
    id: 27_967_096,
    source_type: SourceType::Wikidata,
    name: "Ken's Adlib Music",
    extensions: &["ksm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
