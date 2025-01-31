use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850242: FileFormat = FileFormat {
    id: 105_850_242,
    puid: "wikidata/105850242",
    name: "SMS Coastline data",
    extensions: &["cst"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x41, 0x53, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
