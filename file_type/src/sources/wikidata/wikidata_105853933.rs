use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853933: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_933,
        source_type: SourceType::Wikidata,
        name: "DS Squeeze compressed archive",
        extensions: &["ark"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x46, 0x55, 0x47, 0x48, 0x54, 0x41, 0xD5, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
