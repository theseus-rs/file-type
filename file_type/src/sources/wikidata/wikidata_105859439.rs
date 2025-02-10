use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859439: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_439,
        source_type: SourceType::Wikidata,
        name: "Genstat QTL Data Space",
        extensions: &["qds"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x22, 0x47, 0x65, 0x6E, 0x53, 0x74, 0x61, 0x74, 0x20, 0x51, 0x54, 0x4C,
                        0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20,
                        0x69, 0x6E, 0x64, 0x65, 0x78, 0x20, 0x2D, 0x20, 0x64, 0x6F, 0x20, 0x6E,
                        0x6F, 0x74, 0x20, 0x65, 0x64, 0x69, 0x74, 0x22, 0x0D, 0x0A, 0x56, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
