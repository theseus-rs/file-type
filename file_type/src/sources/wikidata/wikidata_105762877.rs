use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762877: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_877,
        source_type: SourceType::Wikidata,
        name: "XMCD CD information",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x20, 0x78, 0x6D, 0x63, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
