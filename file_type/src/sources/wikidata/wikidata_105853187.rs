use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853187: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_187,
        source_type: SourceType::Wikidata,
        name: "Sound Forge Peak Data File",
        extensions: &["sfk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x46, 0x50, 0x4B, 0x01, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
