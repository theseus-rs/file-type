use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856859: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_859,
        source_type: SourceType::Wikidata,
        name: "GUI Design Studio Catalogue",
        extensions: &["gcat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x43, 0x61, 0x74, 0x61, 0x6C, 0x6F, 0x67, 0x75, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
