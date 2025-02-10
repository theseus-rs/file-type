use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867396: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_396,
        source_type: SourceType::Wikidata,
        name: "MikroTik RouterOS Upgrade Package",
        extensions: &["npk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1E, 0xF1, 0xD0, 0xBA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
