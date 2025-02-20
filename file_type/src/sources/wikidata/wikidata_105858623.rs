use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858623: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_623,
        source_type: SourceType::Wikidata,
        name: "Hard Color Map bitmap",
        extensions: &["hcm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x43, 0x4D, 0x41, 0x38, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
