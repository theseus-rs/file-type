use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850342: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_342,
        source_type: SourceType::Wikidata,
        name: "X.509v3 security certificate",
        extensions: &["crt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
