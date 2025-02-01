use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858069: FileFormat = FileFormat {
    id: 105_858_069,
    puid: "wikidata/105858069",
    name: "Grand Theft Auto IV Item Definition",
    extensions: &["ide"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6F, 0x62, 0x6A, 0x73, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
