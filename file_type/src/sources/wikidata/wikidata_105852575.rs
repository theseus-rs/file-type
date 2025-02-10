use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852575: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_575,
        source_type: SourceType::Wikidata,
        name: "Philips SVCD Designer subtitles",
        extensions: &["sub"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x50, 0x48, 0x49, 0x4C, 0x49, 0x50, 0x53, 0x20, 0x53, 0x56,
                        0x43, 0x44, 0x20, 0x44, 0x45, 0x53, 0x49, 0x47, 0x4E, 0x45, 0x52, 0x20,
                        0x31, 0x2E, 0x35, 0x20, 0x2D, 0x20, 0x32, 0x2E, 0x30, 0x20, 0x53, 0x55,
                        0x42, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x53, 0x20, 0x46, 0x49, 0x4C, 0x45,
                        0x0D, 0x0A, 0x23, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
