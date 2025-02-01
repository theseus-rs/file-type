use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85836636: FileFormat = FileFormat {
    id: 85_836_636,
    puid: "wikidata/85836636",
    name: "Envoy Document File 7",
    extensions: &["evy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB2, 0x97, 0xE1, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
