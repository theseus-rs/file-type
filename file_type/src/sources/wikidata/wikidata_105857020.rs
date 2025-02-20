use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857020: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_020,
        source_type: SourceType::Wikidata,
        name: "GSP Family Tree",
        extensions: &["gft"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0F, 0x00, 0x00, 0x01, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
