use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858330: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_330,
        source_type: SourceType::Wikidata,
        name: "ExeLITE compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x4C, 0x49, 0x54, 0x45, 0x32, 0x0A, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
