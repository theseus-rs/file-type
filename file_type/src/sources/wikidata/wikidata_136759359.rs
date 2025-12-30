use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_136759359: FileType = FileType {
    file_format: &FileFormat {
        id: 136_759_359,
        source_type: SourceType::Wikidata,
        name: "FlatBuffers format",
        extensions: &["tflite"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x46, 0x4C, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
