use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861095: FileFormat = FileFormat {
    id: 105_861_095,
    puid: "wikidata/105861095",
    name: "Linux Software Map entry (gen)",
    extensions: &["lsm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x65, 0x67, 0x69, 0x6E])],
            },
        }],
    }],
    related_formats: &[],
};
