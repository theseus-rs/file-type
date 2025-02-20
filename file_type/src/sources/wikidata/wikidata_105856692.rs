use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856692: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_692,
        source_type: SourceType::Wikidata,
        name: "UltraDesign Font",
        extensions: &["ufnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7C, 0x07, 0x00, 0x14, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
