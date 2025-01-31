use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858525: FileFormat = FileFormat {
    id: 105_858_525,
    puid: "wikidata/105858525",
    name: "IBM Softcopy Reader (Bookmanager) book file",
    extensions: &["bks"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x4B, 0x53, 0x48, 0x45, 0x4C, 0x46, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
