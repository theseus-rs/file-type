use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857024: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_024,
        source_type: SourceType::Wikidata,
        name: "Nakalyne Grape script",
        extensions: &["grape"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x4B, 0x47, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
