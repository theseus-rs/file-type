use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_9135198: FileFormat = FileFormat {
    id: 9_135_198,
    puid: "wikidata/9135198",
    name: "Composer 669 module",
    extensions: &["669"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
