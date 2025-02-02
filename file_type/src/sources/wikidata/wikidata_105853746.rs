use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853746: FileFormat = FileFormat {
    id: 105_853_746,
    source_type: SourceType::Wikidata,
    name: "AND XSynth module",
    extensions: &["amx"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4D, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
