use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862421: FileFormat = FileFormat {
    id: 105_862_421,
    puid: "wikidata/105862421",
    name: "Symphonie Module",
    extensions: &["symmod"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x79, 0x6D, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
