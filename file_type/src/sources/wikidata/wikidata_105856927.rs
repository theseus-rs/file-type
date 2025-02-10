use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856927: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_927,
        source_type: SourceType::Wikidata,
        name: "Reverge Package game data",
        extensions: &["gfs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x52, 0x65, 0x76, 0x65,
                        0x72, 0x67, 0x65, 0x20, 0x50, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x20,
                        0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
