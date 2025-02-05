use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120867987: FileFormat = FileFormat {
    id: 120_867_987,
    source_type: SourceType::Wikidata,
    name: "Cumulus Catalog File",
    extensions: &["ccf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
