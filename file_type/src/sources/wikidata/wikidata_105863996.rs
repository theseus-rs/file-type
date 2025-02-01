use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863996: FileFormat = FileFormat {
    id: 105_863_996,
    puid: "wikidata/105863996",
    name: "MegaZeux world data (generic)",
    extensions: &["mzx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
