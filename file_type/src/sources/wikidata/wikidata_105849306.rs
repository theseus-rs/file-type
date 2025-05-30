use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849306: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_306,
        source_type: SourceType::Wikidata,
        name: "TrainController Animation",
        extensions: &["yra"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0xFE, 0xCA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
