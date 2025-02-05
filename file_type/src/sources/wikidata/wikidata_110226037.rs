use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110226037: FileFormat = FileFormat {
    id: 110_226_037,
    source_type: SourceType::Wikidata,
    name: "Raster Matrix Format",
    extensions: &["rsw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
