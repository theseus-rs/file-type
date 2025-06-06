use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858419: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_419,
        source_type: SourceType::Wikidata,
        name: "Eyeglass format",
        extensions: &["eygl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xBB, 0x0D, 0x0A, 0x65, 0x79, 0x65, 0x67, 0x6C, 0x61, 0x73, 0x73, 0x1A,
                        0x0A, 0xAB,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
