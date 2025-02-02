use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850394: FileFormat = FileFormat {
    id: 105_850_394,
    source_type: SourceType::Wikidata,
    name: "Continuous Wave Accelerometry data",
    extensions: &["cwa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
