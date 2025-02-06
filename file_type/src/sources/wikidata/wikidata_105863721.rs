use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863721: FileFormat = FileFormat {
    id: 105_863_721,
    source_type: SourceType::Wikidata,
    name: "File List Creator list",
    extensions: &["mpd"],
    media_types: &[],
    signatures: &[Signature {
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
