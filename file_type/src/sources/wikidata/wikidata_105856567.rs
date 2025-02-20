use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856567: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_567,
        source_type: SourceType::Wikidata,
        name: "Applause Word data",
        extensions: &["w"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x6F, 0x72, 0x64, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
