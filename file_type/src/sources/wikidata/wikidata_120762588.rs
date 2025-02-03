use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120762588: FileFormat = FileFormat {
    id: 120_762_588,
    source_type: SourceType::Wikidata,
    name: "Topo USA File",
    extensions: &["tpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
