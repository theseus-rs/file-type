use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853243: FileFormat = FileFormat {
    id: 105_853_243,
    source_type: SourceType::Wikidata,
    name: "SoftWrap license data",
    extensions: &["sw2"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x53, 0x57, 0x46, 0x69, 0x6C, 0x65, 0x3E, 0x0D, 0x0A, 0x3C, 0x44, 0x61,
                    0x74, 0x61, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x54, 0x79, 0x70,
                    0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
