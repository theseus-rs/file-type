use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863186: FileFormat = FileFormat {
    id: 105_863_186,
    puid: "wikidata/105863186",
    name: "Midget 3 Instruments",
    extensions: &["mis"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x69, 0x73, 0x02, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
