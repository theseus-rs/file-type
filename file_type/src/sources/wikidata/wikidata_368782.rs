use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_368782: FileType = FileType {
    file_format: &FileFormat {
        id: 368_782,
        source_type: SourceType::Wikidata,
        name: "LHA",
        extensions: &["lha", "lzh"],
        media_types: &["application/x-lzh-compressed"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x6C, 0x68])],
                },
            }],
        }],
        related_formats: &[],
    },
};
