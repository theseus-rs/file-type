use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125364051: FileType = FileType {
    file_format: &FileFormat {
        id: 125_364_051,
        source_type: SourceType::Wikidata,
        name: "Open Brush File",
        extensions: &["tilt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x69, 0x6C, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
