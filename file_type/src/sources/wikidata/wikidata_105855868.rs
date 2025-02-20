use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855868: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_868,
        source_type: SourceType::Wikidata,
        name: "DocuWare XML metafile",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x44, 0x57, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20,
                        0x44, 0x57, 0x35, 0x42, 0x61, 0x73, 0x6B, 0x65, 0x74, 0x46, 0x69, 0x6C,
                        0x65, 0x4E, 0x61, 0x6D, 0x65, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
