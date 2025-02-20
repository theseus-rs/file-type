use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5532331: FileType = FileType {
    file_format: &FileFormat {
        id: 5_532_331,
        source_type: SourceType::Wikidata,
        name: "General content descriptor",
        extensions: &["gcd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x54, 0x79, 0x70, 0x65,
                        0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
