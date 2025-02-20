use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854392: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_392,
        source_type: SourceType::Wikidata,
        name: "Dzip compressed archive (v1.x)",
        extensions: &["dz"],
        media_types: &["application/x-dzip"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x5A, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
