use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856769: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_769,
        source_type: SourceType::Wikidata,
        name: "Unity UnityFS asset bundle",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x6E, 0x69, 0x74, 0x79, 0x46, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
