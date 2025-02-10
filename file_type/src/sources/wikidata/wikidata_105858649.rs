use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858649: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_649,
        source_type: SourceType::Wikidata,
        name: "GoDEX character translation table",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x6F, 0x64, 0x65, 0x78, 0x2C, 0x56, 0x30, 0x30, 0x30, 0x2C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
