use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862934: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_934,
        source_type: SourceType::Wikidata,
        name: "FAC Soundtracker module",
        extensions: &["mus"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x00, 0x80, 0xFF, 0xBF, 0x00, 0x80])],
                },
            }],
        }],
        related_formats: &[],
    },
};
