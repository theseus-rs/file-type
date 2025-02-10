use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862259: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_259,
        source_type: SourceType::Wikidata,
        name: "Video Edit Magic project",
        extensions: &["mpj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x74, 0x69, 0x6D, 0x65, 0x6C, 0x69, 0x6E, 0x65, 0x3E, 0x0D, 0x0A,
                        0x09, 0x3C, 0x67, 0x72, 0x6F, 0x75, 0x70, 0x20, 0x74, 0x79, 0x70, 0x65,
                        0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
