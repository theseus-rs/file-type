use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851215: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_215,
        source_type: SourceType::Wikidata,
        name: "TADS 3 Game",
        extensions: &["t", "t3"],
        media_types: &["application/x-t3vm-image"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x33, 0x2D, 0x69, 0x6D, 0x61, 0x67, 0x65, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
