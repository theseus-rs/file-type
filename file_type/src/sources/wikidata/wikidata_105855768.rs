use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855768: FileFormat = FileFormat {
    id: 105_855_768,
    source_type: SourceType::Wikidata,
    name: "UFOOrbitV2/UFOAnalyzerV2 map",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x46, 0x4F, 0x4F, 0x72, 0x62, 0x69, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
