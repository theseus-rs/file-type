use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762679: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_679,
        source_type: SourceType::Wikidata,
        name: "XXL part program (ASCII)",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x20, 0x44, 0x58, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
