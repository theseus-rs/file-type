use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859732: FileFormat = FileFormat {
    id: 105_859_732,
    puid: "wikidata/105859732",
    name: "Doctor V64 ROM dump",
    extensions: &["v64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x37, 0x80, 0x40, 0x12, 0x00, 0x00, 0x0F, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
