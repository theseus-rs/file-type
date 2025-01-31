use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852013: FileFormat = FileFormat {
    id: 105_852_013,
    puid: "wikidata/105852013",
    name: "Csound Score",
    extensions: &["sco"],
    media_types: &["audio/csound"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0F, 0x43, 0x43, 0x53, 0x43, 0x4F, 0x52, 0x43, 0x48, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
