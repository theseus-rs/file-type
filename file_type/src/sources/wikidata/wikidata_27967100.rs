use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967100: FileFormat = FileFormat {
    id: 27_967_100,
    source_type: SourceType::Wikidata,
    name: "Mario Sequencer file",
    extensions: &["msq"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x43, 0x4F, 0x52, 0x45, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
