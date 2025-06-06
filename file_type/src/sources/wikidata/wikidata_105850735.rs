use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_735,
        source_type: SourceType::Wikidata,
        name: "Karaoke Song List Creator song list",
        extensions: &["ksl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x22, 0x4B, 0x41, 0x52, 0x41, 0x4F, 0x4B, 0x45, 0x20, 0x53, 0x4F, 0x4E,
                        0x47, 0x20, 0x4C, 0x49, 0x53, 0x54, 0x20, 0x43, 0x52, 0x45, 0x41, 0x54,
                        0x4F, 0x52, 0x20, 0x43, 0x44, 0x20, 0x53, 0x45, 0x4C, 0x45, 0x43, 0x54,
                        0x49, 0x4F, 0x4E, 0x20, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
