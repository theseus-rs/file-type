use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967208: FileFormat = FileFormat {
    id: 27_967_208,
    source_type: SourceType::Wikidata,
    name: "Pro Tracker v2.xx module",
    extensions: &["pt2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
