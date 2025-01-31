use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862919: FileFormat = FileFormat {
    id: 105_862_919,
    puid: "wikidata/105862919",
    name: "MVX Module",
    extensions: &["mvm"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x56, 0x4D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
