use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856930: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_930,
        source_type: SourceType::Wikidata,
        name: "Amigaguide hypertext document",
        extensions: &["guide"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
