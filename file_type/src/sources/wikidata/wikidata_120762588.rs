use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120762588: FileFormat = FileFormat {
    id: 120_762_588,
    source_type: SourceType::Wikidata,
    name: "Topo USA File",
    extensions: &["tpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
