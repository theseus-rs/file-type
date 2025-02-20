use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859386: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_386,
        source_type: SourceType::Wikidata,
        name: "Quartus DataBase Info",
        extensions: &["db_info"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x75, 0x61, 0x72, 0x74, 0x75, 0x73, 0x5F, 0x56, 0x65, 0x72, 0x73,
                        0x69, 0x6F, 0x6E, 0x20, 0x3D, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
