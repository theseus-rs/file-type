use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109996995: FileFormat = FileFormat {
    id: 109_996_995,
    source_type: SourceType::Wikidata,
    name: "OrgPlus Template",
    extensions: &["opxt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
