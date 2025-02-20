use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863567: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_567,
        source_type: SourceType::Wikidata,
        name: "SunVox Synthesizer",
        extensions: &["sunsynth"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x53, 0x59, 0x4E, 0x00, 0x00, 0x00, 0x00, 0x56, 0x45, 0x52, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
