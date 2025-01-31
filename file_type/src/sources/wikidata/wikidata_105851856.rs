use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851856: FileFormat = FileFormat {
    id: 105_851_856,
    puid: "wikidata/105851856",
    name: "EPOC Installation package (rel. 2,3,5)",
    extensions: &["sis"],
    media_types: &["x-epoc/x-sisx-app"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6D, 0x00, 0x00, 0x10, 0x19, 0x04, 0x00, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
