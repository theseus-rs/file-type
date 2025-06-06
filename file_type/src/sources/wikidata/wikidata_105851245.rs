use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851245: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_245,
        source_type: SourceType::Wikidata,
        name: "TValue project",
        extensions: &["tv4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC3, 0x54, 0x56, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
