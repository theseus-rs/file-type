use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852263: FileFormat = FileFormat {
    id: 105_852_263,
    puid: "wikidata/105852263",
    name: "SuperREP compressed data",
    extensions: &["srep"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x17, 0x18, 0x35, 0x26, 0x53, 0x52, 0x45, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
