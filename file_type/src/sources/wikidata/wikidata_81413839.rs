use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_81413839: FileFormat = FileFormat {
    id: 81_413_839,
    source_type: SourceType::Wikidata,
    name: "EnCase Case Template",
    extensions: &["CASETEMPLATE"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x6E, 0x46, 0x09, 0x0D, 0x0A, 0xFF, 0x00, 0x43, 0x41, 0x53, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
