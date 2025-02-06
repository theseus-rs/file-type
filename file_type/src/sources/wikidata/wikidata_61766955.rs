use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61766955: FileFormat = FileFormat {
    id: 61_766_955,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 2.0a",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    signatures: &[],
    related_formats: &[],
};
