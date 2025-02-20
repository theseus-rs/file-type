use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856538: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_538,
        source_type: SourceType::Wikidata,
        name: "EViews Workfile",
        extensions: &["wf1"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x77, 0x20, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x54, 0x53, 0x50,
                        0x20, 0x57, 0x6F, 0x72, 0x6B, 0x66, 0x69, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
