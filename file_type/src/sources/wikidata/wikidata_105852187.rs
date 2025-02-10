use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852187: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_187,
        source_type: SourceType::Wikidata,
        name: "SubViewer 2.0 subtitles",
        extensions: &["sub"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x49, 0x4E, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                        0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
