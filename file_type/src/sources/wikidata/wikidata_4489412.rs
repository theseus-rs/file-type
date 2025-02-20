use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4489412: FileType = FileType {
    file_format: &FileFormat {
        id: 4_489_412,
        source_type: SourceType::Wikidata,
        name: "ARFF",
        extensions: &["arff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
