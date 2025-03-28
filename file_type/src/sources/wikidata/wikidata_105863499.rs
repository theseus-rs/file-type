use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863499: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_499,
        source_type: SourceType::Wikidata,
        name: "MusicMatch JukeBox Visualization (v1.0)",
        extensions: &["mvs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x56, 0x53, 0x31, 0x2E, 0x30, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
