use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854923: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_923,
        source_type: SourceType::Wikidata,
        name: "Siva archive (v1)",
        extensions: &["siva"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x42, 0x41, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
