use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856976: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_976,
        source_type: SourceType::Wikidata,
        name: "Genus Graphics Library archive",
        extensions: &["gx", "gxl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0xCA, 0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20,
                        0x28, 0x63, 0x29, 0x20, 0x47, 0x65, 0x6E, 0x75, 0x73, 0x20, 0x4D, 0x69,
                        0x63, 0x72, 0x6F, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x6D, 0x69,
                        0x6E, 0x67, 0x2C, 0x20, 0x49, 0x6E, 0x63, 0x2E, 0x20, 0x31, 0x39, 0x38,
                        0x38, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
