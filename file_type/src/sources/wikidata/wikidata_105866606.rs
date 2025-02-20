use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866606: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_606,
        source_type: SourceType::Wikidata,
        name: "PowerPacker compressed (v2.0)",
        extensions: &["pp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x50, 0x32, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
