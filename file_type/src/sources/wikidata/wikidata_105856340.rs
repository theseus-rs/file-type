use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856340: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_340,
        source_type: SourceType::Wikidata,
        name: "DeskMate worksheet",
        extensions: &["wks"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0E, 0x57, 0x4B, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
