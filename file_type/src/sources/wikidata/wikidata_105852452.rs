use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852452: FileFormat = FileFormat {
    id: 105_852_452,
    source_type: SourceType::Wikidata,
    name: "Spectrum Sound Tracker Pro 2 chiptune",
    extensions: &["stp"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x50, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
