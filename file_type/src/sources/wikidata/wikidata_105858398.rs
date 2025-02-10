use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858398: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_398,
        source_type: SourceType::Wikidata,
        name: "Sweet32 Executable binary",
        extensions: &["swe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x55, 0x53, 0x77, 0x33, 0x32, 0x76, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
