use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857488: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_488,
        source_type: SourceType::Wikidata,
        name: "3-D Professional light",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x4C, 0x49, 0x47, 0x48, 0x54, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
