use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966969: FileFormat = FileFormat {
    id: 27_966_969,
    source_type: SourceType::Wikidata,
    name: "Crystal Caves Sound format",
    extensions: &["snd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
