use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853852: FileFormat = FileFormat {
    id: 105_853_852,
    puid: "wikidata/105853852",
    name: "Verity Collection Index About",
    extensions: &["abt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x63, 0x6F, 0x6E, 0x74, 0x72, 0x6F, 0x6C, 0x3A, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
