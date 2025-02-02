use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_9332294: FileFormat = FileFormat {
    id: 9_332_294,
    source_type: SourceType::Wikidata,
    name: "SubRip text file format",
    extensions: &["srt"],
    media_types: &["application/x-subrip", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0x0D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0x0D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
