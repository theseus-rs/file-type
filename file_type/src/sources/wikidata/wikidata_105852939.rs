use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852939: FileFormat = FileFormat {
    id: 105_852_939,
    source_type: SourceType::Wikidata,
    name: "Opticks Surface",
    extensions: &["srf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x53, 0x55, 0x52, 0x46, 0x41, 0x43, 0x45, 0x41, 0x54, 0x54, 0x52, 0x49,
                    0x42, 0x55, 0x54, 0x45, 0x76, 0x31, 0x2E, 0x30, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
