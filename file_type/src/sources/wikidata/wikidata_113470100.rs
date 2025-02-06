use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113470100: FileFormat = FileFormat {
    id: 113_470_100,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for MS-DOS Document, version 6.0",
    extensions: &["doc"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
