use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860802: FileFormat = FileFormat {
    id: 105_860_802,
    puid: "wikidata/105860802",
    name: "RCFile format",
    extensions: &["rc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x43, 0x46, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
