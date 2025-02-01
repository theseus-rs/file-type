use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863900: FileFormat = FileFormat {
    id: 105_863_900,
    puid: "wikidata/105863900",
    name: "MultiMedia Fusion 2 Application",
    extensions: &["mfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x46, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
