use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857102: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_102,
        source_type: SourceType::Wikidata,
        name: "Allegro MIDI music",
        extensions: &["gro"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x74, 0x72, 0x61, 0x63, 0x6B, 0x20, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
