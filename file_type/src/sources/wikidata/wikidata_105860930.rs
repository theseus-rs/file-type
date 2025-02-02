use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860930: FileFormat = FileFormat {
    id: 105_860_930,
    source_type: SourceType::Wikidata,
    name: "Roland SMF Player Language",
    extensions: &["rlg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4D, 0x46, 0x4C, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
