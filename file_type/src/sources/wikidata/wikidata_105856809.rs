use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856809: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_809,
        source_type: SourceType::Wikidata,
        name: "GPS TrackMaker map",
        extensions: &["gtm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD3, 0x00, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x4D, 0x61, 0x6B, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
