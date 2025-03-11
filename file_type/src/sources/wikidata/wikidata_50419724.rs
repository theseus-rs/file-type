use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50419724: FileType = FileType {
    file_format: &FileFormat {
        id: 50_419_724,
        source_type: SourceType::Wikidata,
        name: "NCSA Hierarchical Data Format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0E, 0x03, 0x13, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
