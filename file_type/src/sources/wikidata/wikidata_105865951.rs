use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865951: FileFormat = FileFormat {
    id: 105_865_951,
    source_type: SourceType::Wikidata,
    name: "PALASM Design Description",
    extensions: &["pds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
