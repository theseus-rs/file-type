use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111530407: FileFormat = FileFormat {
    id: 111_530_407,
    puid: "wikidata/111530407",
    name: "Esri ArcExplorer project file",
    extensions: &["aep"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
