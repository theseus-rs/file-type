use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853513: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_513,
        source_type: SourceType::Wikidata,
        name: "Zoner Draw",
        extensions: &["zmf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x00, 0x00, 0x00, 0xDD, 0x5A, 0x01, 0xD4, 0x78, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
