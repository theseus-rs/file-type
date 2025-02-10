use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_118985600: FileType = FileType {
    file_format: &FileFormat {
        id: 118_985_600,
        source_type: SourceType::Wikidata,
        name: "Simple Relief Format",
        extensions: &["srf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x52, 0x54, 0x43, 0x41, 0x4D, 0x53, 0x52, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
