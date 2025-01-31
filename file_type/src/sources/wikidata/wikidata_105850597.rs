use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850597: FileFormat = FileFormat {
    id: 105_850_597,
    puid: "wikidata/105850597",
    name: "Borland Client Dataset data",
    extensions: &["cds"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x96, 0x19, 0xE0, 0xBD])],
            },
        }],
    }],
    related_formats: &[],
};
