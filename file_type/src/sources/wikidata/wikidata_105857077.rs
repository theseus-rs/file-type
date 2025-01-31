use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857077: FileFormat = FileFormat {
    id: 105_857_077,
    puid: "wikidata/105857077",
    name: "PlayStation RSD 3D Group (gen)",
    extensions: &["grp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x47, 0x52, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
