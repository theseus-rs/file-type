use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762716: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_716,
        source_type: SourceType::Wikidata,
        name: "XSim Save",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x53, 0x69, 0x6D, 0x20, 0x53, 0x61, 0x76, 0x65, 0x2F, 0x52, 0x65,
                        0x73, 0x74, 0x6F, 0x72, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
