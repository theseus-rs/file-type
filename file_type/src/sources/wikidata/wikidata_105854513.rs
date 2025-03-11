use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854513: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_513,
        source_type: SourceType::Wikidata,
        name: "ActiveMARK-protected archive",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x54, 0x4D, 0x53, 0x41, 0x4D, 0x56, 0x4F, 0x48,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
