use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854603: FileFormat = FileFormat {
    id: 105_854_603,
    puid: "wikidata/105854603",
    name: "Lavasoft Ad-Aware reference file",
    extensions: &["ref"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x1B, 0x53, 0x54, 0x44, 0x50, 0x53, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
