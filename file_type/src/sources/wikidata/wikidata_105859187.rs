use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859187: FileFormat = FileFormat {
    id: 105_859_187,
    puid: "wikidata/105859187",
    name: "Bitware BitFax page(s)",
    extensions: &["bfx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x49, 0x54, 0x20, 0x20, 0x46, 0x41, 0x58, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
