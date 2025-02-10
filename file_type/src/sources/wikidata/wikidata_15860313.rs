use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_15860313: FileType = FileType {
    file_format: &FileFormat {
        id: 15_860_313,
        source_type: SourceType::Wikidata,
        name: "Mathematica Notebook",
        extensions: &["nb"],
        media_types: &[
            "application/mathematica",
            "application/vnd.wolfram.mathematica",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x28, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
