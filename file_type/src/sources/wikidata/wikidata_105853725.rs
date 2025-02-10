use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853725: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_725,
        source_type: SourceType::Wikidata,
        name: "Winamp Advanced Visualization Studio File",
        extensions: &["avs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x75, 0x6C, 0x6C, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x41, 0x56, 0x53,
                        0x20, 0x50, 0x72, 0x65, 0x73, 0x65, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
