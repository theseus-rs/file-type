use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855558: FileFormat = FileFormat {
    id: 105_855_558,
    source_type: SourceType::Wikidata,
    name: "Synth Pack module",
    extensions: &["osp"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x42, 0x49, 0x53, 0x59, 0x4E, 0x54, 0x48, 0x50, 0x41, 0x43, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
