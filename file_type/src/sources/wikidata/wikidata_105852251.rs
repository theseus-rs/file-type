use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852251: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_251,
        source_type: SourceType::Wikidata,
        name: "Scid player/event/site/round data",
        extensions: &["sn3", "sn4"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x63, 0x69, 0x64, 0x2E, 0x73, 0x6E, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
