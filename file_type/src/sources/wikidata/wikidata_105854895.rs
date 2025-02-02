use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854895: FileFormat = FileFormat {
    id: 105_854_895,
    source_type: SourceType::Wikidata,
    name: "Hyper archive (compressed)",
    extensions: &["hyp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1A, 0x48, 0x50, 0x25])],
            },
        }],
    }],
    related_formats: &[],
};
