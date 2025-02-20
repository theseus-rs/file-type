use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855045: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_045,
        source_type: SourceType::Wikidata,
        name: "Agros2D document",
        extensions: &["a2d"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x3E, 0x0A, 0x20,
                        0x20, 0x20, 0x20, 0x3C, 0x70, 0x72, 0x6F, 0x62, 0x6C, 0x65, 0x6D, 0x73,
                        0x3E, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x3C, 0x70,
                        0x72, 0x6F, 0x62, 0x6C, 0x65, 0x6D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
