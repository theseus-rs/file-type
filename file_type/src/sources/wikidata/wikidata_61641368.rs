use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61641368: FileFormat = FileFormat {
    id: 61_641_368,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for Windows Document, version 2",
    extensions: &["doc"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
