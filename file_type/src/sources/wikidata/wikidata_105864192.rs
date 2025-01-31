use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864192: FileFormat = FileFormat {
    id: 105_864_192,
    puid: "wikidata/105864192",
    name: "Microsoft PhoneBook (UTF-8)",
    extensions: &["pbk"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x5B])],
            },
        }],
    }],
    related_formats: &[],
};
