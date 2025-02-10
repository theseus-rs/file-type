use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859208: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_208,
        source_type: SourceType::Wikidata,
        name: "NCSA Telnet Interactive Color Raster bitmap",
        extensions: &["icr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1B, 0x5E, 0x57, 0x3B, 0x30, 0x3B, 0x30, 0x3B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
