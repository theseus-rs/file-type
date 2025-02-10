use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855150: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_150,
        source_type: SourceType::Wikidata,
        name: "Font Specifications (with rem)",
        extensions: &["fontspec"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x25])],
                },
            }],
        }],
        related_formats: &[],
    },
};
