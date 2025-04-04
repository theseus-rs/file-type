use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855180: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_180,
        source_type: SourceType::Wikidata,
        name: "Microsoft Fast Find Index v1.x",
        extensions: &["ffx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x28, 0x52,
                        0x29, 0x20, 0x46, 0x75, 0x6C, 0x6C, 0x2D, 0x74, 0x65, 0x78, 0x74, 0x20,
                        0x69, 0x6E, 0x64, 0x65, 0x78, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                        0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
