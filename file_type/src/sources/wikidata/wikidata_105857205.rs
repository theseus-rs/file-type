use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857205: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_205,
        source_type: SourceType::Wikidata,
        name: "HomeBrew Palette",
        extensions: &["hpa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x6F, 0x6D, 0x65, 0x42, 0x72, 0x65, 0x77, 0x20, 0x50, 0x61, 0x6C,
                        0x65, 0x74, 0x74, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
