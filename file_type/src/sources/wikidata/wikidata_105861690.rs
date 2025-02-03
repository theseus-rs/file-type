use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861690: FileFormat = FileFormat {
    id: 105_861_690,
    source_type: SourceType::Wikidata,
    name: "Mass Effect 2 Head Morph",
    extensions: &["me2headmorph"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x49, 0x42, 0x42, 0x45, 0x44, 0x4D, 0x41, 0x53, 0x53, 0x45, 0x46, 0x46,
                    0x45, 0x43, 0x54, 0x32, 0x48, 0x45, 0x41, 0x44, 0x4D, 0x4F, 0x52, 0x50, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
