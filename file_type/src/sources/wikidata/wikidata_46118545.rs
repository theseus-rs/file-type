use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_46118545: FileFormat = FileFormat {
    id: 46_118_545,
    source_type: SourceType::Wikidata,
    name: "Lotus Approach View File",
    extensions: &["apt"],
    media_types: &["application/vnd.lotus-approach"],
    signatures: &[],
    related_formats: &[],
};
