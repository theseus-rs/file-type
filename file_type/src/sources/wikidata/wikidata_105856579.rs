use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856579: FileFormat = FileFormat {
    id: 105_856_579,
    source_type: SourceType::Wikidata,
    name: "wi-scan log",
    extensions: &["std", "sum", "txt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x24, 0x43, 0x72, 0x65, 0x61, 0x74, 0x6F, 0x72, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
