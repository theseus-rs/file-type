use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855403: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_403,
        source_type: SourceType::Wikidata,
        name: "DIV Games Studio Graphics Library",
        extensions: &["fpg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x70, 0x67, 0x1A, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
