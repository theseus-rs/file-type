use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859880: FileFormat = FileFormat {
    id: 105_859_880,
    source_type: SourceType::Wikidata,
    name: "Dahua DVR video",
    extensions: &["dav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x41, 0x48, 0x55, 0x41, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
