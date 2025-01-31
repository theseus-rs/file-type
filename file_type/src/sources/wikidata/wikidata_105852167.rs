use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852167: FileFormat = FileFormat {
    id: 105_852_167,
    puid: "wikidata/105852167",
    name: "Drive SnapShot Disk Image",
    extensions: &["sna"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4E, 0x54, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
