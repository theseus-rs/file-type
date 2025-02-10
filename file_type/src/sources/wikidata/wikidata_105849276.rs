use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849276: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_276,
        source_type: SourceType::Wikidata,
        name: "BYOB sprite",
        extensions: &["ysp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x62, 0x6A, 0x53, 0x01, 0x53, 0x74, 0x63, 0x68, 0x01, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
