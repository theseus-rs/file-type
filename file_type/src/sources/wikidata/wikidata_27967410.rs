use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967410: FileFormat = FileFormat {
    id: 27_967_410,
    source_type: SourceType::Wikidata,
    name: "Creative Voice File",
    extensions: &["voc"],
    media_types: &["audio/x-voc"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x72, 0x65, 0x61, 0x74, 0x69, 0x76, 0x65, 0x20, 0x56, 0x6F, 0x69, 0x63,
                    0x65, 0x20, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
