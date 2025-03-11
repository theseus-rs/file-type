use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762855: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_855,
        source_type: SourceType::Wikidata,
        name: "Egosoft X mesh",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x55, 0x4D, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
