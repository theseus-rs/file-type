use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859538: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_538,
        source_type: SourceType::Wikidata,
        name: "Webcam Video Effects pack",
        extensions: &["vfz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x50, 0x47, 0x01, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
