use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856801: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_801,
        source_type: SourceType::Wikidata,
        name: "Gosset Graphics object",
        extensions: &["gossett"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x6F, 0x6C, 0x79, 0x20, 0x30, 0x0A, 0x20, 0x20, 0x76, 0x65, 0x72,
                        0x74, 0x20, 0x31, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x6C, 0x6F, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
