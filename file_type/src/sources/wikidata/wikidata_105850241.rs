use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850241: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_241,
        source_type: SourceType::Wikidata,
        name: "KTechlab circuit design",
        extensions: &["circuit"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x4B, 0x54,
                        0x65, 0x63, 0x68, 0x6C, 0x61, 0x62, 0x3E, 0x0A, 0x3C, 0x64, 0x6F, 0x63,
                        0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x3D, 0x22,
                        0x63, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x22, 0x20, 0x3E, 0x0A, 0x20,
                        0x3C, 0x69, 0x74, 0x65, 0x6D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
