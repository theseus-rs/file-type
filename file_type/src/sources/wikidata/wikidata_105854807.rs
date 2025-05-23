use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854807: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_807,
        source_type: SourceType::Wikidata,
        name: "Bit Archiver compressed archive",
        extensions: &["bit"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x49, 0x54, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
