use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95985515: FileFormat = FileFormat {
    id: 95_985_515,
    source_type: SourceType::Wikidata,
    name: "Affymetrix PSI file format",
    extensions: &["psi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
