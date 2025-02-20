use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854993: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_993,
        source_type: SourceType::Wikidata,
        name: "Micrognosis compressed archive",
        extensions: &["mar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x61, 0x68, 0x30, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
