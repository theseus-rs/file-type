use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854817: FileFormat = FileFormat {
    id: 105_854_817,
    puid: "wikidata/105854817",
    name: "ChArc compressed archive",
    extensions: &["chz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x43, 0x68, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
