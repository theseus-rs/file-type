use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859741: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_741,
        source_type: SourceType::Wikidata,
        name: "AVF video",
        extensions: &["avf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x56, 0x46, 0x20, 0x57, 0x61, 0x79, 0x6E, 0x65, 0x53, 0x69, 0x6B,
                        0x65, 0x73, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
