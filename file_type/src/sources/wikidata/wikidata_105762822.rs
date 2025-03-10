use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762822: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_822,
        source_type: SourceType::Wikidata,
        name: "Trainz Mesh Importer info",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x74, 0x72, 0x61, 0x69, 0x6E, 0x7A, 0x49, 0x6D, 0x70, 0x6F, 0x72,
                        0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
