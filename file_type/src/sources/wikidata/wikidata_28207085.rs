use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207085: FileFormat = FileFormat {
    id: 28_207_085,
    puid: "wikidata/28207085",
    name: "PrintMaster Graphics file",
    extensions: &["shp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0B, 0x34, 0x58, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
