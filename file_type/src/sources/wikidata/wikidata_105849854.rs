use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849854: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_854,
        source_type: SourceType::Wikidata,
        name: "Lotus 123 configuration (V3)",
        extensions: &["cnf"],
        media_types: &["application/vnd.lotus-1-2-3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x02, 0x14])],
                },
            }],
        }],
        related_formats: &[],
    },
};
