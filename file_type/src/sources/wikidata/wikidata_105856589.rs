use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856589: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_589,
        source_type: SourceType::Wikidata,
        name: "VioLet Composer music",
        extensions: &["wtm"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x73, 0x69, 0x74, 0x69, 0x6F, 0x6E,
                        0x20, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x73, 0x69, 0x74, 0x69, 0x6F, 0x6E,
                        0x5C, 0x73, 0x4E, 0x61, 0x6D, 0x65, 0x3D, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
