use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28756706: FileFormat = FileFormat {
    id: 28_756_706,
    source_type: SourceType::Wikidata,
    name: "Freenet node reference",
    extensions: &["fref"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6F, 0x70, 0x65, 0x6E, 0x6E, 0x65, 0x74, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
