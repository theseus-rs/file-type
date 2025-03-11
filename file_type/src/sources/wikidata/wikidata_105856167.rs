use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856167: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_167,
        source_type: SourceType::Wikidata,
        name: "Dynamic Env. Dynamic Word document",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x2D, 0x2D, 0x20, 0x44, 0x79, 0x6E, 0x61, 0x6D, 0x69, 0x63, 0x20,
                        0x57, 0x6F, 0x72, 0x64, 0x20, 0x2D, 0x2D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
