use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861878: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_878,
        source_type: SourceType::Wikidata,
        name: "NeXtMidas Macro (with rem)",
        extensions: &["mm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
