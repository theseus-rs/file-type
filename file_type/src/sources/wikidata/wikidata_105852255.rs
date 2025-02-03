use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852255: FileFormat = FileFormat {
    id: 105_852_255,
    source_type: SourceType::Wikidata,
    name: "KiXtart SPK notation format",
    extensions: &["spk"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x30, 0x67, 0x32, 0x35, 0x36, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
