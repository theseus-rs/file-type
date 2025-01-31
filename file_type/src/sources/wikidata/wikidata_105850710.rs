use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850710: FileFormat = FileFormat {
    id: 105_850_710,
    puid: "wikidata/105850710",
    name: "Chinese KuGou ResourCe (KuGou Music lyric)",
    extensions: &["krc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6B, 0x72, 0x63, 0x31, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
