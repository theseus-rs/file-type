use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855194: FileFormat = FileFormat {
    id: 105_855_194,
    puid: "wikidata/105855194",
    name: "Future Composer v1.4 module",
    extensions: &["fc"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x43, 0x31, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
