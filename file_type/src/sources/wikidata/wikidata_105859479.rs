use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859479: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_479,
        source_type: SourceType::Wikidata,
        name: "Quartz Composer data",
        extensions: &["qtz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
