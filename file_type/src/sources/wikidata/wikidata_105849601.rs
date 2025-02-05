use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849601: FileFormat = FileFormat {
    id: 105_849_601,
    source_type: SourceType::Wikidata,
    name: "Help File Contents",
    extensions: &["cnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
