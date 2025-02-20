use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85415853: FileType = FileType {
    file_format: &FileFormat {
        id: 85_415_853,
        source_type: SourceType::Wikidata,
        name: "SureThing Project File",
        extensions: &["std"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x56, 0x00, 0xFF, 0x0C, 0x00, 0x12, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
