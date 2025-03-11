use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859492: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_492,
        source_type: SourceType::Wikidata,
        name: "QuickBasic BSAVE binary data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
