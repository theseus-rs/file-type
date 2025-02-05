use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61639409: FileFormat = FileFormat {
    id: 61_639_409,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for Windows Document, version 1",
    extensions: &["doc"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
