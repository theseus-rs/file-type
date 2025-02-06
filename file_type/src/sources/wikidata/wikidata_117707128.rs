use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117707128: FileFormat = FileFormat {
    id: 117_707_128,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Project file",
    extensions: &["dtp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
