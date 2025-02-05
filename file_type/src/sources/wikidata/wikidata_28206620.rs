use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206620: FileFormat = FileFormat {
    id: 28_206_620,
    source_type: SourceType::Wikidata,
    name: "Microsoft Paint, version 2",
    extensions: &["msp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x69, 0x6E, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
