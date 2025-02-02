use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58103077: FileFormat = FileFormat {
    id: 58_103_077,
    source_type: SourceType::Wikidata,
    name: "LifeTechnologies SDS",
    extensions: &["sds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
