use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865975: FileFormat = FileFormat {
    id: 105_865_975,
    source_type: SourceType::Wikidata,
    name: "ArtCAM Palette",
    extensions: &["pal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x20, 0x20, 0x41, 0x72, 0x74, 0x43, 0x41, 0x4D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
