use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854171: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_171,
        source_type: SourceType::Wikidata,
        name: "The Sims Archive",
        extensions: &["far"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x41, 0x52, 0x21, 0x62, 0x79, 0x41, 0x5A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
