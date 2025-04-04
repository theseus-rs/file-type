use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852100: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_100,
        source_type: SourceType::Wikidata,
        name: "Surfplan kite project",
        extensions: &["sle"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3A, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
