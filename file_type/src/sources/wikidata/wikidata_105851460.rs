use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851460: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_460,
        source_type: SourceType::Wikidata,
        name: "TheDraw design (v3.x)",
        extensions: &["td"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x15, 0x54, 0x68, 0x65, 0x44, 0x72, 0x61, 0x77, 0x20, 0x53, 0x61, 0x76,
                        0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2E, 0x0D, 0x0A, 0x1A, 0x04, 0x33,
                        0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
