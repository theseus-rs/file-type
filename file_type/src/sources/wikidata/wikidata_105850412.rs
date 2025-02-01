use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850412: FileFormat = FileFormat {
    id: 105_850_412,
    puid: "wikidata/105850412",
    name: "PaintShow Font",
    extensions: &["chr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
