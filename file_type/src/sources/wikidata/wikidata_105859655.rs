use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859655: FileFormat = FileFormat {
    id: 105_859_655,
    source_type: SourceType::Wikidata,
    name: "VisiCalc spreadsheet (alt)",
    extensions: &["vc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x57, 0x31, 0x0D, 0x0A, 0x2F])],
            },
        }],
    }],
    related_formats: &[],
};
