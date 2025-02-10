use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850714: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_714,
        source_type: SourceType::Wikidata,
        name: "KbdEdit Export(ed) layout",
        extensions: &["kbe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x13, 0x00, 0x4B, 0x62, 0x64, 0x45, 0x64, 0x69, 0x74, 0x20, 0x65, 0x78,
                        0x70, 0x6F, 0x72, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
