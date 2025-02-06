use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855293: FileFormat = FileFormat {
    id: 105_855_293,
    source_type: SourceType::Wikidata,
    name: "Gravis Firebird configuration",
    extensions: &["fbd"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x69, 0x72, 0x65, 0x62, 0x69, 0x72, 0x64, 0x20, 0x43, 0x6F, 0x6E, 0x66,
                    0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x46, 0x69, 0x6C,
                    0x65, 0x2C, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x31, 0x2E, 0x30,
                    0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
