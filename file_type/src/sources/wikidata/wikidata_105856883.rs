use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856883: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_883,
        source_type: SourceType::Wikidata,
        name: "GameBoy Sound System GBR dump",
        extensions: &["gbr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x42, 0x52, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
