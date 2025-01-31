use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863676: FileFormat = FileFormat {
    id: 105_863_676,
    puid: "wikidata/105863676",
    name: "MacWrite II document",
    extensions: &["mcw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x2E, 0x00, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
