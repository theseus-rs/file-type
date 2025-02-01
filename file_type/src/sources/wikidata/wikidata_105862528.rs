use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862528: FileFormat = FileFormat {
    id: 105_862_528,
    puid: "wikidata/105862528",
    name: "Maker Interchange Format Book",
    extensions: &["mif"],
    media_types: &["application/vnd.mif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x42, 0x6F, 0x6F, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
