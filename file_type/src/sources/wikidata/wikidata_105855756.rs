use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855756: FileFormat = FileFormat {
    id: 105_855_756,
    source_type: SourceType::Wikidata,
    name: "DeScribe document",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x04, 0x00, 0x44, 0x65, 0x53, 0x63, 0x72, 0x69, 0x62, 0x65, 0x20,
                    0x20, 0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x44, 0x65,
                    0x53, 0x63, 0x72, 0x69, 0x62, 0x65, 0x2C, 0x20, 0x49, 0x6E, 0x63, 0x2E, 0x2C,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
