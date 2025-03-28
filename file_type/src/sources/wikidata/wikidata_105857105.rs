use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857105: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_105,
        source_type: SourceType::Wikidata,
        name: "G-code (generated by Slic3r)",
        extensions: &["gcode"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x20, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20,
                        0x62, 0x79, 0x20, 0x53, 0x6C, 0x69, 0x63, 0x33, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
