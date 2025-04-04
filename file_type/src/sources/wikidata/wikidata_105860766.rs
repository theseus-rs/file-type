use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860766: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_766,
        source_type: SourceType::Wikidata,
        name: "MDL Reaction format",
        extensions: &["rxn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x24, 0x52, 0x58, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
