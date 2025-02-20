use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853606: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_606,
        source_type: SourceType::Wikidata,
        name: "Zoot information processor database",
        extensions: &["zot"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x4F, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
