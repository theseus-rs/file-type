use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865261: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_261,
        source_type: SourceType::Wikidata,
        name: "NeoBook for DOS document",
        extensions: &["pub"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x6F, 0x42, 0x6F, 0x6F, 0x6B, 0x20, 0x44, 0x6F, 0x63, 0x75,
                        0x6D, 0x65, 0x6E, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
