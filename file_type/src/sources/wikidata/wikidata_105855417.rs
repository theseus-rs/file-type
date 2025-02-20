use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855417: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_417,
        source_type: SourceType::Wikidata,
        name: "STK Facility Network",
        extensions: &["fn"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x4E, 0x65, 0x74, 0x77, 0x6F, 0x72,
                        0x6B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
