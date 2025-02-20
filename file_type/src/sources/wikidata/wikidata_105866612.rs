use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866612: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_612,
        source_type: SourceType::Wikidata,
        name: "PSXjin movie capture",
        extensions: &["pjm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4A, 0x4D, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
