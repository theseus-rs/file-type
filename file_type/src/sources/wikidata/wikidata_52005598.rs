use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52005598: FileFormat = FileFormat {
    id: 52_005_598,
    source_type: SourceType::Wikidata,
    name: "AMI Draw Vector Image",
    extensions: &["sdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
