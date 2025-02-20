use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858184: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_184,
        source_type: SourceType::Wikidata,
        name: "Exploring Pascal compiled H-Code (v1.0)",
        extensions: &["exx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB7, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
