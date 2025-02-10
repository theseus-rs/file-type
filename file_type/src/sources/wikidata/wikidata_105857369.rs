use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857369: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_369,
        source_type: SourceType::Wikidata,
        name: "Trackjoy GUS Tracker module",
        extensions: &["joy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x52, 0x41, 0x43, 0x4B, 0x4A, 0x4F, 0x59, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
