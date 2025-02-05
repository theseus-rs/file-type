use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863854: FileFormat = FileFormat {
    id: 105_863_854,
    source_type: SourceType::Wikidata,
    name: "Macrocell format",
    extensions: &["mc"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x4D, 0x32, 0x5D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
