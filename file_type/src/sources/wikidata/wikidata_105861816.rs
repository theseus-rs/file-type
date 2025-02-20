use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861816: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_816,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money auto saved data",
        extensions: &["mny"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xBA, 0xBA, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
