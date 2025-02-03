use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852829: FileFormat = FileFormat {
    id: 105_852_829,
    source_type: SourceType::Wikidata,
    name: "Spectrum emulator snapshot",
    extensions: &["sp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
