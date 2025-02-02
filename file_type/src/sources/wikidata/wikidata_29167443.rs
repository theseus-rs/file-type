use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167443: FileFormat = FileFormat {
    id: 29_167_443,
    source_type: SourceType::Wikidata,
    name: "OME-TIFF",
    extensions: &["ome.tiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
