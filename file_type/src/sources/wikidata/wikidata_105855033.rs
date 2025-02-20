use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855033: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_033,
        source_type: SourceType::Wikidata,
        name: "AirZip FileSECURE format (print quality)",
        extensions: &["azf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x5A, 0x46, 0x17])],
                },
            }],
        }],
        related_formats: &[],
    },
};
