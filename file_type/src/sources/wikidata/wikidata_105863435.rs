use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863435: FileFormat = FileFormat {
    id: 105_863_435,
    source_type: SourceType::Wikidata,
    name: "Extensible Binary Meta Language / Matroska stream",
    extensions: &["mka", "mkv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x45, 0xDF, 0xA3])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x45, 0xDF, 0xA3])],
                },
            }],
        },
    ],
    related_formats: &[],
};
