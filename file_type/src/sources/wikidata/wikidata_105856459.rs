use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856459: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_459,
        source_type: SourceType::Wikidata,
        name: "Wintrac log data (v3)",
        extensions: &["wtf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x69, 0x6E, 0x74, 0x72, 0x61, 0x63, 0x20, 0x33, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
