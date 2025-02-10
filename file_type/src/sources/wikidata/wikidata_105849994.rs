use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849994: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_994,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM C-Crypt protected (v1.02)",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE9])],
                },
            }],
        }],
        related_formats: &[],
    },
};
