use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109996995: FileFormat = FileFormat {
    id: 109_996_995,
    source_type: SourceType::Wikidata,
    name: "OrgPlus Template",
    extensions: &["opxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
