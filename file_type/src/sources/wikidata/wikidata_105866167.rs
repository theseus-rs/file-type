use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866167: FileFormat = FileFormat {
    id: 105_866_167,
    puid: "wikidata/105866167",
    name: "Turbo Pascal Symbol Table",
    extensions: &["psm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
