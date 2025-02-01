use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862769: FileFormat = FileFormat {
    id: 105_862_769,
    puid: "wikidata/105862769",
    name: "OctaMED Music Editor module (v2.10)",
    extensions: &["med4"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x45, 0x44, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
