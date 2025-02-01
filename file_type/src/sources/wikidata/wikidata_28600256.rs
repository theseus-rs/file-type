use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600256: FileFormat = FileFormat {
    id: 28_600_256,
    puid: "wikidata/28600256",
    name: "ASCII Encoded HP 48 Object",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x25, 0x48, 0x50, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
