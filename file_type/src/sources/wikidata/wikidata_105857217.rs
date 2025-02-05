use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857217: FileFormat = FileFormat {
    id: 105_857_217,
    source_type: SourceType::Wikidata,
    name: "ALICE: The Personal Pascal Program Help",
    extensions: &["huf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFF, 0x03, 0x24, 0x22, 0x20, 0x00, 0x2C, 0x20, 0x00, 0x2C, 0x48, 0x70,
                    0x6E, 0x6F, 0x64, 0x65, 0x2F, 0x00, 0x2C, 0x48, 0x70, 0x73, 0x79, 0x6D, 0x62,
                    0x6F, 0x6C, 0x2F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
