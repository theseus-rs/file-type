use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862672: FileFormat = FileFormat {
    id: 105_862_672,
    puid: "wikidata/105862672",
    name: "MBasic source",
    extensions: &["mbi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x42, 0x4D, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
