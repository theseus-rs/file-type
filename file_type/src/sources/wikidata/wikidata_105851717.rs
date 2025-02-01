use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851717: FileFormat = FileFormat {
    id: 105_851_717,
    puid: "wikidata/105851717",
    name: "Siag spreadsheet (with rem)",
    extensions: &["siag"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
