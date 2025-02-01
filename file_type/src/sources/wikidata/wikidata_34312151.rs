use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34312151: FileFormat = FileFormat {
    id: 34_312_151,
    puid: "wikidata/34312151",
    name: "Macromedia Director, uncompressed PC variant",
    extensions: &["dir"],
    media_types: &["application/x-director"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x39, 0x56, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
