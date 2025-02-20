use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855191: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_191,
        source_type: SourceType::Wikidata,
        name: "BluffTitler Effect",
        extensions: &["fx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2F, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
