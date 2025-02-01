use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967193: FileFormat = FileFormat {
    id: 27_967_193,
    puid: "wikidata/27967193",
    name: "Graoumf Tracker 2 module",
    extensions: &["gt2"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x54, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
