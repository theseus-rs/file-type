use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7978630: FileType = FileType {
    file_format: &FileFormat {
        id: 7_978_630,
        source_type: SourceType::Wikidata,
        name: "Webarchive",
        extensions: &["webarchive"],
        media_types: &["application/x-webarchive"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
