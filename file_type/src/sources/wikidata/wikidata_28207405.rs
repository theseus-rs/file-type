use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207405: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_405,
        source_type: SourceType::Wikidata,
        name: "TLG",
        extensions: &["tlg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x4C, 0x47, 0x30, 0x2E, 0x30, 0x00, 0x73, 0x64, 0x73, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
