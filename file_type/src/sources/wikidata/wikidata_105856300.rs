use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856300: FileFormat = FileFormat {
    id: 105_856_300,
    puid: "wikidata/105856300",
    name: "22DISK format Definition (with notes)",
    extensions: &["def"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x4F, 0x54, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
