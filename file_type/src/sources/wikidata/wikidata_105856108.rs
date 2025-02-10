use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856108: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_108,
        source_type: SourceType::Wikidata,
        name: "Visual Basic Active Designer file",
        extensions: &["dsr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x35, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
