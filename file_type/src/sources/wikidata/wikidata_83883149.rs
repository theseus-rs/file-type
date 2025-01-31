use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83883149: FileFormat = FileFormat {
    id: 83_883_149,
    puid: "wikidata/83883149",
    name: "Electronically Certified Document",
    extensions: &["edoc"],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
