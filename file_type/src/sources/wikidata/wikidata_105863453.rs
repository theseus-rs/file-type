use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863453: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_453,
        source_type: SourceType::Wikidata,
        name: "Okuma CNC program",
        extensions: &["min"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x24])],
                },
            }],
        }],
        related_formats: &[],
    },
};
