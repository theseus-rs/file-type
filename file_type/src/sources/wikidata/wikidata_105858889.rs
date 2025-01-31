use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858889: FileFormat = FileFormat {
    id: 105_858_889,
    puid: "wikidata/105858889",
    name: "QuickLink II Fax bitmap (old)",
    extensions: &["qfx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4C, 0x49, 0x49, 0x46, 0x41, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
