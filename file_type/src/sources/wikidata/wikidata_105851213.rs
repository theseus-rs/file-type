use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851213: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_213,
        source_type: SourceType::Wikidata,
        name: "Telegram Desktop Data File (gen)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x44, 0x46, 0x24])],
                },
            }],
        }],
        related_formats: &[],
    },
};
