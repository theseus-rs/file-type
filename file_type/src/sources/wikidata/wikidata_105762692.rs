use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762692: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_692,
        source_type: SourceType::Wikidata,
        name: "SMS XY3 Series (old)",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x59, 0x33, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
