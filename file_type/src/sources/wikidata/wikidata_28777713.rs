use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777713: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_713,
        source_type: SourceType::Wikidata,
        name: "NetImmerse file format",
        extensions: &["nif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x61, 0x6D, 0x65, 0x62, 0x72, 0x79, 0x6F, 0x20, 0x46, 0x69, 0x6C,
                        0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2C, 0x20, 0x56, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
