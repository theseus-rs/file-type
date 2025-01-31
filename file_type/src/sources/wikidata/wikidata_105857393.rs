use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857393: FileFormat = FileFormat {
    id: 105_857_393,
    puid: "wikidata/105857393",
    name: "Japanese Word Processor document",
    extensions: &["jwp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x02, 0x26, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
