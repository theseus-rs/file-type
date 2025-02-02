use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117707128: FileFormat = FileFormat {
    id: 117_707_128,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Project file",
    extensions: &["dtp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
