use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855905: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_905,
        source_type: SourceType::Wikidata,
        name: "Turbo C Context",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x75, 0x72, 0x62, 0x6F, 0x20, 0x43, 0x20, 0x43, 0x6F, 0x6E, 0x74,
                        0x65, 0x78, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
