use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858599: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_599,
        source_type: SourceType::Wikidata,
        name: "TIM2 PlayStation2 bitmap",
        extensions: &["tm2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x49, 0x4D, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
