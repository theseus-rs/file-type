use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849969: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_969,
        source_type: SourceType::Wikidata,
        name: "Rad Cad drawing",
        extensions: &["cad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x02, 0xDF, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
