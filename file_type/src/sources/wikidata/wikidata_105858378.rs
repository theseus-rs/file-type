use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858378: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_378,
        source_type: SourceType::Wikidata,
        name: "Eclipse product marker",
        extensions: &["eclipseproduct"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x45, 0x63, 0x6C, 0x69, 0x70, 0x73, 0x65, 0x20, 0x50, 0x72, 0x6F,
                        0x64, 0x75, 0x63, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
