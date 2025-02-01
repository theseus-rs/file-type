use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73514196: FileFormat = FileFormat {
    id: 73_514_196,
    puid: "wikidata/73514196",
    name: "MegaCAD Project",
    extensions: &["prt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x65, 0x67, 0x61, 0x43, 0x61, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
