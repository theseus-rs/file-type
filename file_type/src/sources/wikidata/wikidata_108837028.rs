use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108837028: FileFormat = FileFormat {
    id: 108_837_028,
    source_type: SourceType::Wikidata,
    name: "Nero CD EXTRA Compilation",
    extensions: &["nrm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
