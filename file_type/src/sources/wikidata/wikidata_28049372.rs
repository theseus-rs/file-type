use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049372: FileFormat = FileFormat {
    id: 28_049_372,
    source_type: SourceType::Wikidata,
    name: "ComputerEyes Raw Data Format, low resolution",
    extensions: &["ce1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x59, 0x45, 0x53, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
