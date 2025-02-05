use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856377: FileFormat = FileFormat {
    id: 105_856_377,
    source_type: SourceType::Wikidata,
    name: "Oxford Instruments electron microscope image tile",
    extensions: &["dmp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x34, 0x15, 0xA1, 0x04, 0x00, 0x6E, 0x30, 0x00, 0x01, 0x00, 0x00, 0x00, 0x78,
                    0x9C, 0xEC, 0xDD,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
