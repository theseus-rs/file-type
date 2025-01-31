use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863733: FileFormat = FileFormat {
    id: 105_863_733,
    puid: "wikidata/105863733",
    name: "Octalyser 6-channel STe/Falcon Module",
    extensions: &["mod"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x44, 0x36, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
