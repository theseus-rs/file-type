use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862524: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_524,
        source_type: SourceType::Wikidata,
        name: "DIV Games Studio Map",
        extensions: &["map"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x61, 0x70, 0x1A, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
