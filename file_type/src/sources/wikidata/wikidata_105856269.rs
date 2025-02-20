use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856269: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_269,
        source_type: SourceType::Wikidata,
        name: "DataBase Professional database",
        extensions: &["db"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x42, 0x70, 0x72, 0x6F, 0x20, 0x64, 0x61, 0x74, 0x61, 0x66, 0x69,
                        0x6C, 0x65, 0x09,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
