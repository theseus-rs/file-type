use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966969: FileFormat = FileFormat {
    id: 27_966_969,
    source_type: SourceType::Wikidata,
    name: "Crystal Caves Sound format",
    extensions: &["snd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
