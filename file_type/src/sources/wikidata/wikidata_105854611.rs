use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854611: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_611,
        source_type: SourceType::Wikidata,
        name: "Norton pcAnywhere Automated Procedures",
        extensions: &["ap6"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x6F, 0x72, 0x74, 0x6F, 0x6E, 0x20, 0x70, 0x63, 0x41, 0x4E, 0x59,
                        0x57, 0x48, 0x45, 0x52, 0x45, 0x20, 0x41, 0x75, 0x74, 0x6F, 0x6D, 0x61,
                        0x74, 0x65, 0x64, 0x20, 0x50, 0x72, 0x6F, 0x63, 0x65, 0x64, 0x75, 0x72,
                        0x65, 0x73, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
