use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854888: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_888,
        source_type: SourceType::Wikidata,
        name: "Alpha Four record Set",
        extensions: &["set"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x34, 0x09, 0x0A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
