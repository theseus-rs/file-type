use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849836: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_836,
        source_type: SourceType::Wikidata,
        name: "CleWin CIF layout",
        extensions: &["cif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x43, 0x49, 0x46, 0x20, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6E,
                        0x20, 0x62, 0x79, 0x20, 0x43, 0x6C, 0x65, 0x57, 0x69, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
