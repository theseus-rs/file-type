use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851965: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_965,
        source_type: SourceType::Wikidata,
        name: "Simple Vector Format (v1.10)",
        extensions: &["svf"],
        media_types: &["image/vnd.svf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x56, 0x46, 0x20, 0x76, 0x31, 0x2E, 0x31, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
