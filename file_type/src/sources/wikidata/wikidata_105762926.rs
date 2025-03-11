use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762926: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_926,
        source_type: SourceType::Wikidata,
        name: "X1R / X Millennium Save State",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x6D, 0x69, 0x6C, 0x5F, 0x5F, 0x73, 0x74, 0x61, 0x74, 0x65, 0x5F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
