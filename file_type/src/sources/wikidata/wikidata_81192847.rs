use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81192847: FileType = FileType {
    file_format: &FileFormat {
        id: 81_192_847,
        source_type: SourceType::Wikidata,
        name: "The Bee Archiver compressed archive",
        extensions: &["bee"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x65, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
