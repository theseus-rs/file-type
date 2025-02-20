use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859756: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_756,
        source_type: SourceType::Wikidata,
        name: "VBOX data",
        extensions: &["vbo"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
                        0x20, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
