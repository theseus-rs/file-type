use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862610: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_610,
        source_type: SourceType::Wikidata,
        name: "Gateway Music",
        extensions: &["mus"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA1, 0x05, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
