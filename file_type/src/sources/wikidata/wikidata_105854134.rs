use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854134: FileFormat = FileFormat {
    id: 105_854_134,
    puid: "wikidata/105854134",
    name: "MAr compressed archive",
    extensions: &["mar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x72, 0x30, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
