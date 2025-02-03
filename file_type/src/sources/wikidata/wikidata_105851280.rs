use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851280: FileFormat = FileFormat {
    id: 105_851_280,
    source_type: SourceType::Wikidata,
    name: "TurboCAD drawing (v1 Win)",
    extensions: &["tcw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x75, 0x72, 0x62, 0x6F, 0x43, 0x41, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
