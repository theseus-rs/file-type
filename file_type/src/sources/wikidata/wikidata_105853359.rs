use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853359: FileFormat = FileFormat {
    id: 105_853_359,
    puid: "wikidata/105853359",
    name: "SQL Server CE Edition database",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x53, 0x53, 0x43, 0x45, 0x20,
                    0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
