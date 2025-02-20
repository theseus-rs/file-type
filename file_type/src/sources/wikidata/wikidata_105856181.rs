use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856181: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_181,
        source_type: SourceType::Wikidata,
        name: "Delta RPM Package",
        extensions: &["drpm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xED, 0xAB, 0xEE, 0xDB, 0x03, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
