use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849940: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_940,
        source_type: SourceType::Wikidata,
        name: "Celestia Sphere displacement Mesh",
        extensions: &["cms"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x70, 0x68, 0x65, 0x72, 0x65, 0x44, 0x69, 0x73, 0x70, 0x6C, 0x61,
                        0x63, 0x65, 0x6D, 0x65, 0x6E, 0x74, 0x4D, 0x65, 0x73, 0x68,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
