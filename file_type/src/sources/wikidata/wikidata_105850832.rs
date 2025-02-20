use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850832: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_832,
        source_type: SourceType::Wikidata,
        name: "Key Home Gourmet Cookbook",
        extensions: &["kcf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x65, 0x79, 0x20, 0x48, 0x6F, 0x6D, 0x65, 0x20, 0x47, 0x6F, 0x75,
                        0x72, 0x6D, 0x65, 0x74, 0x20, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x43, 0x6F,
                        0x6F, 0x6B, 0x62, 0x6F, 0x6F, 0x6B, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
