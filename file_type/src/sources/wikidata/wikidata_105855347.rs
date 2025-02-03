use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855347: FileFormat = FileFormat {
    id: 105_855_347,
    source_type: SourceType::Wikidata,
    name: "form-Z compiled script",
    extensions: &["fsb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x5A, 0x53, 0x43, 0x00, 0x00, 0x00, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
