use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207313: FileFormat = FileFormat {
    id: 28_207_313,
    puid: "wikidata/28207313",
    name: "Run length encoded True Colour Picture",
    extensions: &["tre"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x72, 0x65, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
