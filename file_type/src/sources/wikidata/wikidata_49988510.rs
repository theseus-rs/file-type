use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49988510: FileFormat = FileFormat {
    id: 49_988_510,
    puid: "wikidata/49988510",
    name: "Rocket Book eBook format",
    extensions: &["rb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB0, 0x0C, 0xB0, 0x0C])],
            },
        }],
    }],
    related_formats: &[],
};
