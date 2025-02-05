use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61777529: FileFormat = FileFormat {
    id: 61_777_529,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 3",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    signatures: &[],
    related_formats: &[],
};
