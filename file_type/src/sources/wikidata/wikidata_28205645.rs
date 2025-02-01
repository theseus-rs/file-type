use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205645: FileFormat = FileFormat {
    id: 28_205_645,
    puid: "wikidata/28205645",
    name: "XV thumbnail",
    extensions: &["p7"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x37, 0x20, 0x33, 0x33, 0x32, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
