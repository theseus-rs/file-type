use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850125: FileFormat = FileFormat {
    id: 105_850_125,
    source_type: SourceType::Wikidata,
    name: "Ease Calcform spreadsheet",
    extensions: &["cal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x50, 0x45, 0x52, 0x41, 0x1A, 0x0A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
