use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855545: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_545,
        source_type: SourceType::Wikidata,
        name: "Oberon/F Code File",
        extensions: &["ocf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x43, 0x4F, 0x6F, 0x0A, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
