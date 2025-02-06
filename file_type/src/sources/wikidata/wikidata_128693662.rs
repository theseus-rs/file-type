use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128693662: FileFormat = FileFormat {
    id: 128_693_662,
    source_type: SourceType::Wikidata,
    name: "Befunge file format",
    extensions: &["befunge"],
    media_types: &["application/x-befunge"],
    signatures: &[],
    related_formats: &[],
};
