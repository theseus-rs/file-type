use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857309: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_309,
        source_type: SourceType::Wikidata,
        name: "Hexels drawing",
        extensions: &["hxl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x45, 0x58, 0x45, 0x4C, 0x53, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
