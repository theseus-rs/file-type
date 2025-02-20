use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854154: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_154,
        source_type: SourceType::Wikidata,
        name: "Anno Designer layout",
        extensions: &["ad"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x7B, 0x22, 0x43, 0x6F, 0x6C, 0x6F, 0x72, 0x22, 0x3A, 0x7B, 0x22,
                        0x41, 0x22, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
