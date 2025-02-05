use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967190: FileFormat = FileFormat {
    id: 27_967_190,
    source_type: SourceType::Wikidata,
    name: "General Digital Music module",
    extensions: &["gdm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
