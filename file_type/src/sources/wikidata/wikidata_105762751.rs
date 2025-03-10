use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762751: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_751,
        source_type: SourceType::Wikidata,
        name: "HP Xine compressed",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x69, 0x6E, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
