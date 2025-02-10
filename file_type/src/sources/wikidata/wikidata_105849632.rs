use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849632: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_632,
        source_type: SourceType::Wikidata,
        name: "EACA Colour Genie virtual tape image",
        extensions: &["cas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x6C, 0x6F, 0x75, 0x72, 0x20, 0x47, 0x65, 0x6E, 0x69, 0x65,
                        0x20, 0x2D, 0x20, 0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x20, 0x54,
                        0x61, 0x70, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0A, 0x0D, 0x4E, 0x61,
                        0x6D, 0x65, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
