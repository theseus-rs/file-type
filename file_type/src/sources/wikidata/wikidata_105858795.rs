use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858795: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_795,
        source_type: SourceType::Wikidata,
        name: "Photo Enote (Enot) external photo viewer settings",
        extensions: &["bws"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x48, 0x4F, 0x54, 0x4F, 0x56, 0x49, 0x45, 0x57, 0x45, 0x52,
                        0x53, 0x45, 0x54, 0x54, 0x49, 0x4E, 0x47, 0x53, 0x5D, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
