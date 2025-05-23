use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865784: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_784,
        source_type: SourceType::Wikidata,
        name: "Yahoo! Voice Mail",
        extensions: &["post"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x21, 0x56, 0x4D, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
