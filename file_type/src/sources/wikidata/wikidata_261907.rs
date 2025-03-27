use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_261907: FileType = FileType {
    file_format: &FileFormat {
        id: 261_907,
        source_type: SourceType::Wikidata,
        name: "eXperimental Computing Facility",
        extensions: &["xcf"],
        media_types: &["image/x-xcf", "image/xcf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x67, 0x69, 0x6D, 0x70, 0x20, 0x78, 0x63, 0x66, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
