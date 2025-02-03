use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850823: FileFormat = FileFormat {
    id: 105_850_823,
    source_type: SourceType::Wikidata,
    name: "KiXtart tokenized script (with password)",
    extensions: &["kx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1A, 0xAF, 0x06, 0x00, 0x01, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
