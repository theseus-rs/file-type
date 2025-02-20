use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849617: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_617,
        source_type: SourceType::Wikidata,
        name: "WhatsApp encrypted database",
        extensions: &["crypt7"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x01, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
