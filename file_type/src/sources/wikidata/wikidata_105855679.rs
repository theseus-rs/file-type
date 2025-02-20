use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855679: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_679,
        source_type: SourceType::Wikidata,
        name: "6502 binary relocation format (v2)",
        extensions: &["o65"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x00, 0x6F, 0x36, 0x35, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
