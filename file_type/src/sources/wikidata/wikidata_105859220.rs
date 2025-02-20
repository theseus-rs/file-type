use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859220: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_220,
        source_type: SourceType::Wikidata,
        name: "Bleeper Music Maker music",
        extensions: &["bmm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x6C, 0x65, 0x65, 0x70, 0x65, 0x72, 0x20, 0x4D, 0x75, 0x73, 0x69,
                        0x63, 0x20, 0x4D, 0x61, 0x6B, 0x65, 0x72, 0x20, 0x62, 0x79, 0x20, 0x52,
                        0x6F, 0x62, 0x62, 0x69, 0x2D, 0x39, 0x38, 0x35, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x0D, 0x0A, 0x52, 0x65,
                        0x76, 0x69, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
