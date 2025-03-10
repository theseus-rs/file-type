use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857035: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_035,
        source_type: SourceType::Wikidata,
        name: "ISO G-Code program",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x30, 0x20, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x50, 0x47, 0x4D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
