use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850412: FileFormat = FileFormat {
    id: 105_850_412,
    source_type: SourceType::Wikidata,
    name: "PaintShow Font",
    extensions: &["chr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
