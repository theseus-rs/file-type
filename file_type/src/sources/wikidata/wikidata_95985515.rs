use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95985515: FileFormat = FileFormat {
    id: 95_985_515,
    source_type: SourceType::Wikidata,
    name: "Affymetrix PSI file format",
    extensions: &["psi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
