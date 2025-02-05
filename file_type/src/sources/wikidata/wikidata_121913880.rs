use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121913880: FileFormat = FileFormat {
    id: 121_913_880,
    source_type: SourceType::Wikidata,
    name: "Memory Stick Voice File ADPCM Codec",
    extensions: &["msv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
