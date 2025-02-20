use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853219: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_219,
        source_type: SourceType::Wikidata,
        name: "Speech Synthesis Markup Language",
        extensions: &["ssml"],
        media_types: &["application/ssml+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x73, 0x70, 0x65, 0x61, 0x6B, 0x20, 0x78, 0x6D, 0x6C, 0x3A, 0x6C,
                        0x61, 0x6E, 0x67, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
