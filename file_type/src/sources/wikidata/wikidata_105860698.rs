use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860698: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_698,
        source_type: SourceType::Wikidata,
        name: "RamSoft's ZX Spectrum Replay",
        extensions: &["rzx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x5A, 0x58, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
