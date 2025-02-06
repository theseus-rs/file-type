use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111530407: FileFormat = FileFormat {
    id: 111_530_407,
    source_type: SourceType::Wikidata,
    name: "Esri ArcExplorer project file",
    extensions: &["aep"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
