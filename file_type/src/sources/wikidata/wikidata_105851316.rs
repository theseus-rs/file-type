use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851316: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_316,
        source_type: SourceType::Wikidata,
        name: "Xcode Text Based Definition",
        extensions: &["tbd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x2D, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
