use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967188: FileFormat = FileFormat {
    id: 27_967_188,
    puid: "wikidata/27967188",
    name: "Future Composer (BSI) module",
    extensions: &["bsi"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x55, 0x43, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
