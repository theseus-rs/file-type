use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855791: FileFormat = FileFormat {
    id: 105_855_791,
    source_type: SourceType::Wikidata,
    name: "FL Studio Drum Patch (v1)",
    extensions: &["dmpatch"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x31, 0x50, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
