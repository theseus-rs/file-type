use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858338: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_338,
        source_type: SourceType::Wikidata,
        name: "Eagle preferences",
        extensions: &["epf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x45, 0x61, 0x67, 0x6C, 0x65, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
