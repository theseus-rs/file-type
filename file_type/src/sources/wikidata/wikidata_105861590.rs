use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861590: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_590,
        source_type: SourceType::Wikidata,
        name: "TML BASIC Library",
        extensions: &["lib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x4D, 0x4C, 0x42, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
