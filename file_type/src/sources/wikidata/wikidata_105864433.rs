use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864433: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_433,
        source_type: SourceType::Wikidata,
        name: "Movie Setter Project",
        extensions: &["prod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x78, 0x53, 0x63, 0x65, 0x6E, 0x65, 0x45, 0x64, 0x69, 0x74, 0x6F, 0x72,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
