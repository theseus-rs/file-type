use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850372: FileFormat = FileFormat {
    id: 105_850_372,
    source_type: SourceType::Wikidata,
    name: "WildMidi Configuration",
    extensions: &["cfg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x64, 0x69, 0x72, 0x20, 0x50, 0x52, 0x4F, 0x47, 0x44, 0x49, 0x52, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
