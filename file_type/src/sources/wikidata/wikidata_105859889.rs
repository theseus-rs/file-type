use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859889: FileFormat = FileFormat {
    id: 105_859_889,
    puid: "wikidata/105859889",
    name: "VZ200/300 image (type F0)",
    extensions: &["vz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x5A, 0x46, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
