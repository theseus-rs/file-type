use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858284: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_284,
        source_type: SourceType::Wikidata,
        name: "LOB compressed Amiga executable (type 2)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x4C, 0x4F, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
