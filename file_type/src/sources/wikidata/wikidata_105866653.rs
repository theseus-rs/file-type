use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866653: FileFormat = FileFormat {
    id: 105_866_653,
    source_type: SourceType::Wikidata,
    name: "Psion Serie 5/EPOC Word document",
    extensions: &["psi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x37, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10, 0x7F, 0x00, 0x00, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
