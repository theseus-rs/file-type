use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855076: FileFormat = FileFormat {
    id: 105_855_076,
    source_type: SourceType::Wikidata,
    name: "Hyper archive (stored)",
    extensions: &["hyp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1A, 0x53, 0x54, 0x25])],
            },
        }],
    }],
    related_formats: &[],
};
