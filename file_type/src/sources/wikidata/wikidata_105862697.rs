use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862697: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_697,
        source_type: SourceType::Wikidata,
        name: "Maxwell Render Scene",
        extensions: &["mxs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF9, 0xB2, 0x2A, 0xD1])],
                },
            }],
        }],
        related_formats: &[],
    },
};
