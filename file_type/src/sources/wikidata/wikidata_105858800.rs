use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858800: FileFormat = FileFormat {
    id: 105_858_800,
    puid: "wikidata/105858800",
    name: "PCX bitmap (v2.5)",
    extensions: &["pcx"],
    media_types: &["image/vnd.zbrush.pcx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
