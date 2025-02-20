use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855723: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_723,
        source_type: SourceType::Wikidata,
        name: "Painter 3D model",
        extensions: &["d"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x6C, 0x65, 0x74, 0x74, 0x65, 0x3A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
