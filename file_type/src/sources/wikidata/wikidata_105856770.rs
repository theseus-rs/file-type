use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856770: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_770,
        source_type: SourceType::Wikidata,
        name: "Universal Communications Format",
        extensions: &["ucf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x50, 0x44, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
