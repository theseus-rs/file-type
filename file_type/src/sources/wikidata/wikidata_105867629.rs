use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867629: FileFormat = FileFormat {
    id: 105_867_629,
    puid: "wikidata/105867629",
    name: "NEC JIS encoded file",
    extensions: &["nec"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
