use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860705: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_705,
        source_type: SourceType::Wikidata,
        name: "LEGO Mindstorms NXT Executable",
        extensions: &["rxe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x6E, 0x64, 0x73, 0x74, 0x6F, 0x72, 0x6D, 0x73, 0x4E, 0x58,
                        0x54, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
