use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854930: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_930,
        source_type: SourceType::Wikidata,
        name: "Final Fantasy Game Archive",
        extensions: &["lgp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x53, 0x51, 0x55, 0x41, 0x52, 0x45, 0x53, 0x4F, 0x46, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
