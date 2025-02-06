use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967208: FileFormat = FileFormat {
    id: 27_967_208,
    source_type: SourceType::Wikidata,
    name: "Pro Tracker v2.xx module",
    extensions: &["pt2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
