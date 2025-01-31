use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858369: FileFormat = FileFormat {
    id: 105_858_369,
    puid: "wikidata/105858369",
    name: "UN/EDIFACT",
    extensions: &["edi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
