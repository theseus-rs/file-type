use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863814: FileFormat = FileFormat {
    id: 105_863_814,
    puid: "wikidata/105863814",
    name: "MinGW Developer Studio Project",
    extensions: &["mdsp"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
