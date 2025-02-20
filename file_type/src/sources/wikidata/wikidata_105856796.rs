use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856796: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_796,
        source_type: SourceType::Wikidata,
        name: "Klik'n'Play game",
        extensions: &["gam"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x41, 0x4D, 0x45, 0x26, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
