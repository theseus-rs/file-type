use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854452: FileFormat = FileFormat {
    id: 105_854_452,
    puid: "wikidata/105854452",
    name: "QazaR compressed file",
    extensions: &["aza"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3F, 0x32, 0x32, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
