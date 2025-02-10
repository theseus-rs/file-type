use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857683: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_683,
        source_type: SourceType::Wikidata,
        name: "UDI Disk Image",
        extensions: &["udi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x44, 0x49, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
