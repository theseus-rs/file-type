use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855540: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_540,
        source_type: SourceType::Wikidata,
        name: "3D Construction Kit Object",
        extensions: &["obj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x42, 0x4A, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
