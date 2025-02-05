use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205944: FileFormat = FileFormat {
    id: 28_205_944,
    source_type: SourceType::Wikidata,
    name: "Doré Raster",
    extensions: &["dore", "img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
