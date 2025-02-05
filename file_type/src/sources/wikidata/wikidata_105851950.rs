use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851950: FileFormat = FileFormat {
    id: 105_851_950,
    source_type: SourceType::Wikidata,
    name: "StorageCraft ShadowProtect backup image",
    extensions: &["spf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x46, 0x49, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
