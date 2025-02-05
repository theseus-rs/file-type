use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113674482: FileFormat = FileFormat {
    id: 113_674_482,
    source_type: SourceType::Wikidata,
    name: "3D Landscape 2.0 File",
    extensions: &["3sl", "lnd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
