use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851404: FileFormat = FileFormat {
    id: 105_851_404,
    puid: "wikidata/105851404",
    name: "RamTracker module",
    extensions: &["trk"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x52, 0x4B, 0x30, 0x31, 0x2F, 0x54, 0x56, 0x2E, 0x45, 0x53, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
