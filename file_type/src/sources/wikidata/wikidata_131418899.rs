use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131418899: FileFormat = FileFormat {
    id: 131_418_899,
    source_type: SourceType::Wikidata,
    name: "Web IDL file format",
    extensions: &["webidl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
