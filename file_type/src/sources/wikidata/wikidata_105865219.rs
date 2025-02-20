use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865219: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_219,
        source_type: SourceType::Wikidata,
        name: "ArchiCAD Plan Archive",
        extensions: &["pla", "pln"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x4F, 0x46, 0x20, 0x46, 0x44, 0x42, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
