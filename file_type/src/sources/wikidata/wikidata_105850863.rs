use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850863: FileFormat = FileFormat {
    id: 105_850_863,
    puid: "wikidata/105850863",
    name: "Kea Coloring Book page",
    extensions: &["kcx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x19, 0x83, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
