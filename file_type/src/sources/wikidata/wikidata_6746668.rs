use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6746668: FileType = FileType {
    file_format: &FileFormat {
        id: 6_746_668,
        source_type: SourceType::Wikidata,
        name: "Management Information Format",
        extensions: &["mif"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x63, 0x6F, 0x6D, 0x70, 0x6F, 0x6E,
                        0x65, 0x6E, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
