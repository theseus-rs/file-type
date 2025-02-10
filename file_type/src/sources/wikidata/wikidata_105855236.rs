use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855236: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_236,
        source_type: SourceType::Wikidata,
        name: "PerFROM Pro Form",
        extensions: &["frl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x65, 0x72, 0x46, 0x4F, 0x52, 0x4D, 0x20, 0x50, 0x52, 0x4F, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
