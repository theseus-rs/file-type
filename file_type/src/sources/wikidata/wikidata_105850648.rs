use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850648: FileFormat = FileFormat {
    id: 105_850_648,
    source_type: SourceType::Wikidata,
    name: "Korg Trinity/Triton multisample",
    extensions: &["kmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x50, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
