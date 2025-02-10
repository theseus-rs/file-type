use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859834: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_834,
        source_type: SourceType::Wikidata,
        name: "LABView Virtual Instrument",
        extensions: &["vi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x53, 0x52, 0x43, 0x0D, 0x0A, 0x00, 0x03, 0x4C, 0x56, 0x49, 0x4E,
                        0x4C, 0x42, 0x56, 0x57, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
