use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862426: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_426,
        source_type: SourceType::Wikidata,
        name: "MovieShop timeline",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCB, 0x04, 0xEB, 0x04, 0x00, 0x14, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
