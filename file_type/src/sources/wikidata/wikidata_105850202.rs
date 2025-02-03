use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850202: FileFormat = FileFormat {
    id: 105_850_202,
    source_type: SourceType::Wikidata,
    name: "BAMZOOKi Contest",
    extensions: &["contest"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x22, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6F, 0x75, 0x72, 0x73, 0x22, 0x20,
                    0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
