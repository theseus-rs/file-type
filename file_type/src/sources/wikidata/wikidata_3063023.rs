use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3063023: FileFormat = FileFormat {
    id: 3_063_023,
    source_type: SourceType::Wikidata,
    name: "FASTQ format",
    extensions: &["fastq", "fq"],
    media_types: &["text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x53, 0x45, 0x51, 0x5F, 0x49, 0x44])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x53, 0x45, 0x51, 0x5F, 0x49, 0x44])],
                },
            }],
        },
    ],
    related_formats: &[],
};
