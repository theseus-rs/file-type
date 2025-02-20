use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859984: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_984,
        source_type: SourceType::Wikidata,
        name: "VZ200/300 image (type F1)",
        extensions: &["vz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x20, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
