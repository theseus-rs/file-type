use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859021: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_021,
        source_type: SourceType::Wikidata,
        name: "Message string storage",
        extensions: &["bmg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x45, 0x53, 0x47, 0x62, 0x6D, 0x67, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
