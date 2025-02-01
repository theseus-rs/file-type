use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863352: FileFormat = FileFormat {
    id: 105_863_352,
    puid: "wikidata/105863352",
    name: "Yamaha Midimonitor Messages",
    extensions: &["msg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x4D, 0x53, 0x47, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
