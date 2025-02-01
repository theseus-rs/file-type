use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205507: FileFormat = FileFormat {
    id: 28_205_507,
    puid: "wikidata/28205507",
    name: "GlowIcons",
    extensions: &["info"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE3, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
