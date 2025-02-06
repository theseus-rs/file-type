use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52005598: FileFormat = FileFormat {
    id: 52_005_598,
    source_type: SourceType::Wikidata,
    name: "AMI Draw Vector Image",
    extensions: &["sdw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
