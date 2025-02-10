use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863736: FileFormat = FileFormat {
    id: 105_863_736,
    source_type: SourceType::Wikidata,
    name: "MuSiCa text music format (with rem)",
    extensions: &["msd"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B])],
            },
        }],
    }],
    related_formats: &[],
};
