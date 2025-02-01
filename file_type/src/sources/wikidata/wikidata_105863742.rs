use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863742: FileFormat = FileFormat {
    id: 105_863_742,
    puid: "wikidata/105863742",
    name: "MIDITracker music",
    extensions: &["mtr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x74, 0x72, 0x4D, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
