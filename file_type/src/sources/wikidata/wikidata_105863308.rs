use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863308: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_308,
        source_type: SourceType::Wikidata,
        name: "Real-Time Sound Processor II MIDI",
        extensions: &["mid"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x54, 0x53, 0x50, 0x2E, 0x4D, 0x49, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
