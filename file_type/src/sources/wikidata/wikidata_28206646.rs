use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206646: FileFormat = FileFormat {
    id: 28_206_646,
    puid: "wikidata/28206646",
    name: "Multi Palette Picture",
    extensions: &["mpp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
