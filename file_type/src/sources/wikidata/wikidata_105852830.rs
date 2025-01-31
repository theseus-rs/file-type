use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852830: FileFormat = FileFormat {
    id: 105_852_830,
    puid: "wikidata/105852830",
    name: "Drive SnapShot Disk Image (additional parts)",
    extensions: &["sn1"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4E, 0x43, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
