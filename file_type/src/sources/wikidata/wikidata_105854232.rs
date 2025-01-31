use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854232: FileFormat = FileFormat {
    id: 105_854_232,
    puid: "wikidata/105854232",
    name: "BMA Archiver compressed archive",
    extensions: &["bma"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x42, 0x4D, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
