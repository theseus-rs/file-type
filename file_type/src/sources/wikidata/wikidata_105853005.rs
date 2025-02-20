use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853005: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_005,
        source_type: SourceType::Wikidata,
        name: "Sampbox 4 Macro",
        extensions: &["sam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x53, 0x61,
                        0x6D, 0x70, 0x62, 0x6F, 0x78, 0x34, 0x20, 0x6D, 0x61, 0x63, 0x72, 0x6F,
                        0x20, 0x66, 0x69, 0x6C, 0x65, 0x2E, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
