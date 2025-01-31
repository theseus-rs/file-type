use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851520: FileFormat = FileFormat {
    id: 105_851_520,
    puid: "wikidata/105851520",
    name: "CS1er debugger exported data",
    extensions: &["txt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B, 0x52, 0x65, 0x63, 0x6F, 0x72, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
