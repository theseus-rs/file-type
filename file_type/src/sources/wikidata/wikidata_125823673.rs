use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125823673: FileFormat = FileFormat {
    id: 125_823_673,
    source_type: SourceType::Wikidata,
    name: "Gzipped Tar File",
    extensions: &["tgz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
