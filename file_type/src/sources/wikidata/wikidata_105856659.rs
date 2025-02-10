use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856659: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_659,
        source_type: SourceType::Wikidata,
        name: "Unreal Music",
        extensions: &["umx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC1, 0x83, 0x2A, 0x9E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
