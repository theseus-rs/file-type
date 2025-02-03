use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121913880: FileFormat = FileFormat {
    id: 121_913_880,
    source_type: SourceType::Wikidata,
    name: "Memory Stick Voice File ADPCM Codec",
    extensions: &["msv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
