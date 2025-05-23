use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851945: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_945,
        source_type: SourceType::Wikidata,
        name: "TechnoSound Turbo samples set",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x41, 0x4E, 0x4B, 0x00, 0x00, 0x00, 0x00, 0x53, 0x4D, 0x50, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
