use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76143366: FileType = FileType {
    file_format: &FileFormat {
        id: 76_143_366,
        source_type: SourceType::Wikidata,
        name: "TeX Virtual Font",
        extensions: &["vf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF7, 0xCA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
