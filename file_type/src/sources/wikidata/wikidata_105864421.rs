use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864421: FileFormat = FileFormat {
    id: 105_864_421,
    source_type: SourceType::Wikidata,
    name: "Papyrus document",
    extensions: &["pap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x50, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
