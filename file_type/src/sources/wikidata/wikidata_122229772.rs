use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122229772: FileFormat = FileFormat {
    id: 122_229_772,
    source_type: SourceType::Wikidata,
    name: "Digital Interface Format",
    extensions: &["dif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
