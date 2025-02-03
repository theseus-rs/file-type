use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859108: FileFormat = FileFormat {
    id: 105_859_108,
    source_type: SourceType::Wikidata,
    name: "Speccy eXtended Graphics bitmap",
    extensions: &["sxg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0x53, 0x58, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
