use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859706: FileFormat = FileFormat {
    id: 105_859_706,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts MAD video (low-quality-inter-frame)",
    extensions: &["mad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x44, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
