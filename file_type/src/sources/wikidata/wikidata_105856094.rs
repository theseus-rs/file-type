use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856094: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_094,
        source_type: SourceType::Wikidata,
        name: "DVDShrink 3.1 deep analysis data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x23, 0x23, 0x44, 0x56, 0x44, 0x53, 0x48, 0x52, 0x49, 0x4E, 0x4B,
                        0x33, 0x2E, 0x31, 0x00, 0x1C, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
