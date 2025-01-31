use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853917: FileFormat = FileFormat {
    id: 105_853_917,
    puid: "wikidata/105853917",
    name: "ZipGenius encrypted compressed archive",
    extensions: &["czip"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x5A, 0x49, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
