use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866647: FileFormat = FileFormat {
    id: 105_866_647,
    source_type: SourceType::Wikidata,
    name: "Polygon File Format (binary)",
    extensions: &["ply"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x6C, 0x79])],
            },
        }],
    }],
    related_formats: &[],
};
