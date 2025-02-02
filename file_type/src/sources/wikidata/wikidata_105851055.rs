use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851055: FileFormat = FileFormat {
    id: 105_851_055,
    source_type: SourceType::Wikidata,
    name: "TeamBG Infinity Engine data format (v4)",
    extensions: &["tbg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x42, 0x47, 0x34, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
