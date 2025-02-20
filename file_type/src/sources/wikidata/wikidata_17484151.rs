use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17484151: FileType = FileType {
    file_format: &FileFormat {
        id: 17_484_151,
        source_type: SourceType::Wikidata,
        name: "Scribus Document",
        extensions: &["scd", "scd.gz", "sla", "sla.gz", "slaz"],
        media_types: &["application/vnd.scribus"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
