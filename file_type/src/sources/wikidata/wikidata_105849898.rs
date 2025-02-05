use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849898: FileFormat = FileFormat {
    id: 105_849_898,
    source_type: SourceType::Wikidata,
    name: "Cal3D Skeleton File",
    extensions: &["csf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
