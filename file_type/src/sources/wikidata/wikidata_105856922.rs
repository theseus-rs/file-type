use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856922: FileFormat = FileFormat {
    id: 105_856_922,
    puid: "wikidata/105856922",
    name: "Raptor GLB encrypted container",
    extensions: &["glb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x9B, 0xD1, 0x09])],
            },
        }],
    }],
    related_formats: &[],
};
