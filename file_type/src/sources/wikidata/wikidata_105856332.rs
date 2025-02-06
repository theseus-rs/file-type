use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856332: FileFormat = FileFormat {
    id: 105_856_332,
    source_type: SourceType::Wikidata,
    name: "Mozilla Mail folder cache",
    extensions: &["dat"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x20, 0x3C, 0x21, 0x2D, 0x2D, 0x20, 0x3C, 0x6D, 0x64, 0x62, 0x3A,
                    0x6D, 0x6F, 0x72, 0x6B, 0x3A, 0x7A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
