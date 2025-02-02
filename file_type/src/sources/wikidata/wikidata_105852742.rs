use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852742: FileFormat = FileFormat {
    id: 105_852_742,
    source_type: SourceType::Wikidata,
    name: "Windows Shadow spooler (NT)",
    extensions: &["shd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x49, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
