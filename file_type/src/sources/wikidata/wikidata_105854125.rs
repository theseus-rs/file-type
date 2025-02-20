use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854125: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_125,
        source_type: SourceType::Wikidata,
        name: "Ay Emul Skin (v2.0)",
        extensions: &["ays"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x79, 0x5F, 0x45, 0x6D, 0x75, 0x6C, 0x20, 0x32, 0x2E, 0x30, 0x20,
                        0x53, 0x6B, 0x69, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
