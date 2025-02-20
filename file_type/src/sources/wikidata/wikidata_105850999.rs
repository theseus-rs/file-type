use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850999: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_999,
        source_type: SourceType::Wikidata,
        name: "Terragen Object geometry",
        extensions: &["tgo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x45, 0x52, 0x52, 0x41, 0x47, 0x45, 0x4E, 0x47, 0x45, 0x4F, 0x4D,
                        0x45, 0x54, 0x52, 0x59, 0x44, 0x41, 0x54, 0x41, 0x1C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
