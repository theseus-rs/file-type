use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851468: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_468,
        source_type: SourceType::Wikidata,
        name: "TI-Nspire CAS OS image",
        extensions: &["tnc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x49, 0x2D, 0x4E, 0x73, 0x70, 0x69, 0x72, 0x65, 0x2E, 0x74, 0x6E,
                        0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
