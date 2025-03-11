use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863289: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_289,
        source_type: SourceType::Wikidata,
        name: "Amiga Arts SoundMonitor instrument",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4D, 0x4F, 0x4E, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x05, 0x49, 0x4E,
                        0x53, 0x54, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x42, 0x4F, 0x44, 0x59,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
