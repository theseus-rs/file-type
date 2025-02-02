use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126811644: FileFormat = FileFormat {
    id: 126_811_644,
    source_type: SourceType::Wikidata,
    name: "Fenix Graphics Collection File",
    extensions: &["fpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
