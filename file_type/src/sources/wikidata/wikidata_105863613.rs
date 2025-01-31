use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863613: FileFormat = FileFormat {
    id: 105_863_613,
    puid: "wikidata/105863613",
    name: "Max Payne data file",
    extensions: &["ras"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x41, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
