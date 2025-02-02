use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863832: FileFormat = FileFormat {
    id: 105_863_832,
    source_type: SourceType::Wikidata,
    name: "Quartus Memory Initialization File",
    extensions: &["mif"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
