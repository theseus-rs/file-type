use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856068: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_068,
        source_type: SourceType::Wikidata,
        name: "DeSmuME DataBase",
        extensions: &["ddb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x65, 0x53, 0x6D, 0x75, 0x4D, 0x45, 0x20, 0x64, 0x61, 0x74, 0x61,
                        0x62, 0x61, 0x73, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
