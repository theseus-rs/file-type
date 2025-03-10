use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862746: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_746,
        source_type: SourceType::Wikidata,
        name: "M music (v1.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x6D, 0x69, 0x67, 0x61, 0x4D, 0x31, 0x30, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
