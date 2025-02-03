use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855574: FileFormat = FileFormat {
    id: 105_855_574,
    source_type: SourceType::Wikidata,
    name: "Elite: Dangerous game data",
    extensions: &["ovl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x52, 0x45, 0x53, 0x00, 0x12, 0x00, 0x01, 0x94, 0x20, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
