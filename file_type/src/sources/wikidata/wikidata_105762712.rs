use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762712: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_712,
        source_type: SourceType::Wikidata,
        name: "Waveme diagram",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x44, 0x69, 0x61, 0x67, 0x72, 0x61, 0x6D, 0x20, 0x41, 0x70, 0x70,
                        0x72, 0x6F, 0x76, 0x65, 0x5F, 0x71, 0x75, 0x69, 0x74, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
