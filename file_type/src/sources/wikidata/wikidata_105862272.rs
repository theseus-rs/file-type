use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862272: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_272,
        source_type: SourceType::Wikidata,
        name: "Moonbase game data",
        extensions: &["mb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x6F, 0x6F, 0x6E, 0x62, 0x61, 0x73, 0x65, 0x20, 0x76, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
