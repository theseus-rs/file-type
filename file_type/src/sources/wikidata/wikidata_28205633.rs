use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205633: FileFormat = FileFormat {
    id: 28_205_633,
    source_type: SourceType::Wikidata,
    name: "SuperJPG thumbnail cache",
    extensions: &["tnc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x00, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
