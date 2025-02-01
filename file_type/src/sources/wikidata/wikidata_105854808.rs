use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854808: FileFormat = FileFormat {
    id: 105_854_808,
    puid: "wikidata/105854808",
    name: "SBX SpinnerBaker eXtractor compressed archive",
    extensions: &["sb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x31, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
