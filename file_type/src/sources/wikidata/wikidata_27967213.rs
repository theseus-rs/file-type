use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967213: FileFormat = FileFormat {
    id: 27_967_213,
    source_type: SourceType::Wikidata,
    name: "Real Tracker module",
    extensions: &["rtm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
