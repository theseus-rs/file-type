use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867087: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_087,
        source_type: SourceType::Wikidata,
        name: "Audacity Nyquist plug-in",
        extensions: &["ny"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x6E, 0x79, 0x71, 0x75, 0x69, 0x73, 0x74, 0x20, 0x70, 0x6C, 0x75,
                        0x67, 0x2D, 0x69, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
