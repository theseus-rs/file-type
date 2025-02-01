use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863149: FileFormat = FileFormat {
    id: 105_863_149,
    puid: "wikidata/105863149",
    name: "Multimedia Viewer Book",
    extensions: &["mvb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3F, 0x5F, 0x03, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
