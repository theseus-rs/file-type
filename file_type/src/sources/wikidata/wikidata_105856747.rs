use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856747: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_747,
        source_type: SourceType::Wikidata,
        name: "Unity UnityWeb asset bundle",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x6E, 0x69, 0x74, 0x79, 0x57, 0x65, 0x62,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
