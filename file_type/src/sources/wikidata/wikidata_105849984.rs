use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849984: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_984,
        source_type: SourceType::Wikidata,
        name: "Cabri 3D Language",
        extensions: &["cgl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20, 0x43, 0x61, 0x62,
                        0x72, 0x69, 0x49, 0x49, 0x20, 0x76, 0x65, 0x72, 0x73, 0x2E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
