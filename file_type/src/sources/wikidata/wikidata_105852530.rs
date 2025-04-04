use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852530: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_530,
        source_type: SourceType::Wikidata,
        name: "3D-XplorMath Surface format",
        extensions: &["surf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61, 0x46,
                        0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
