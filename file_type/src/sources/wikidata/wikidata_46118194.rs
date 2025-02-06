use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_46118194: FileFormat = FileFormat {
    id: 46_118_194,
    source_type: SourceType::Wikidata,
    name: "Lotus Approach View File, version 97",
    extensions: &["apr"],
    media_types: &["application/vnd.lotus-approach"],
    signatures: &[],
    related_formats: &[],
};
