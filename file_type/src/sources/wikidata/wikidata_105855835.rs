use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855835: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_835,
        source_type: SourceType::Wikidata,
        name: "Norton Textra Writer Document (v2.x)",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0xFD, 0xFF, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x13,
                        0x08, 0xFF, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
