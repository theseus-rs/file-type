use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850210: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_210,
        source_type: SourceType::Wikidata,
        name: "WinOnCD Project",
        extensions: &["cpj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x57, 0x69, 0x6E, 0x4F, 0x6E, 0x43, 0x44, 0x20, 0x50, 0x72, 0x6F,
                        0x6A, 0x65, 0x63, 0x74, 0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
