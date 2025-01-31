use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863721: FileFormat = FileFormat {
    id: 105_863_721,
    puid: "wikidata/105863721",
    name: "File List Creator list",
    extensions: &["mpd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
