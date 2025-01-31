use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73515266: FileFormat = FileFormat {
    id: 73_515_266,
    puid: "wikidata/73515266",
    name: "Protege Project",
    extensions: &["pprj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
