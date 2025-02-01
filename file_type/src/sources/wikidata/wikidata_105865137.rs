use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865137: FileFormat = FileFormat {
    id: 105_865_137,
    puid: "wikidata/105865137",
    name: "PERQemu Hard Disk image",
    extensions: &["phd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x45, 0x52, 0x51])],
            },
        }],
    }],
    related_formats: &[],
};
