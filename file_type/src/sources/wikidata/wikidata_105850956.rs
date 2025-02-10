use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850956: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_956,
        source_type: SourceType::Wikidata,
        name: "Trackjoy GUS Tracker song",
        extensions: &["tjs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x52, 0x41, 0x43, 0x4B, 0x4A, 0x4F, 0x59, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
