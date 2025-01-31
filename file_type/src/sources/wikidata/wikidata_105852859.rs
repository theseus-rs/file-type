use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852859: FileFormat = FileFormat {
    id: 105_852_859,
    puid: "wikidata/105852859",
    name: "Sequential Vibes Music",
    extensions: &["svm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x45, 0x51, 0x56, 0x49, 0x42, 0x45, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
