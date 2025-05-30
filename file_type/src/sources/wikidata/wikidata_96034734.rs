use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96034734: FileType = FileType {
    file_format: &FileFormat {
        id: 96_034_734,
        source_type: SourceType::Wikidata,
        name: "Graphlet file format",
        extensions: &["gml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x72, 0x61, 0x70, 0x68, 0x20, 0x5B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
