use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167443: FileFormat = FileFormat {
    id: 29_167_443,
    source_type: SourceType::Wikidata,
    name: "OME-TIFF",
    extensions: &["ome.tiff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
