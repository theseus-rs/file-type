use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856142: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_142,
        source_type: SourceType::Wikidata,
        name: "WPSpell Dictionary",
        extensions: &["dct"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x57, 0x50, 0x53, 0x70, 0x65, 0x6C, 0x6C, 0x20, 0x44, 0x69, 0x63,
                        0x74, 0x69, 0x6F, 0x6E, 0x61, 0x72, 0x79, 0x20, 0x2D, 0x20, 0x77, 0x77,
                        0x77, 0x2E, 0x77, 0x70, 0x63, 0x75, 0x62, 0x65, 0x64, 0x2E, 0x63, 0x6F,
                        0x6D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
