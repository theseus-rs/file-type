use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861253: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_253,
        source_type: SourceType::Wikidata,
        name: "LinkTreeNote document",
        extensions: &["ltn"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x2D, 0x2D, 0x7C, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x56,
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x33, 0x2D, 0x2D, 0x3E, 0x0D,
                        0x0A, 0x3C, 0x21, 0x2D, 0x2D, 0x7C, 0x44, 0x61, 0x74, 0x61, 0x4C, 0x6F,
                        0x63, 0x61, 0x6C, 0x65, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
