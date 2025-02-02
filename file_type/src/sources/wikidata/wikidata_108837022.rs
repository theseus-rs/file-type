use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108837022: FileFormat = FileFormat {
    id: 108_837_022,
    source_type: SourceType::Wikidata,
    name: "Nero Mixed Mode CD Compilation",
    extensions: &["nrm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
