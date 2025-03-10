use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762885: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_885,
        source_type: SourceType::Wikidata,
        name: "XeX page definition",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x3C, 0x73, 0x69, 0x74, 0x65, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
