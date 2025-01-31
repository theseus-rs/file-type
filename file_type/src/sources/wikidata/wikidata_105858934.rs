use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858934: FileFormat = FileFormat {
    id: 105_858_934,
    puid: "wikidata/105858934",
    name: "PCX bitmap (v2.8, palette)",
    extensions: &["pcx"],
    media_types: &["image/vnd.zbrush.pcx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
