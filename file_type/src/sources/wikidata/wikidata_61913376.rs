use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61913376: FileFormat = FileFormat {
    id: 61_913_376,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for Macintosh Document, version 1",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    signatures: &[],
    related_formats: &[],
};
