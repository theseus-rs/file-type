use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861046: FileFormat = FileFormat {
    id: 105_861_046,
    source_type: SourceType::Wikidata,
    name: "System V 64-bit library",
    extensions: &["a"],
    media_types: &["application/x-archive"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E, 0x0A, 0x2F, 0x53, 0x59, 0x4D, 0x36,
                    0x34, 0x2F, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
