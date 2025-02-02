use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865022: FileFormat = FileFormat {
    id: 105_865_022,
    source_type: SourceType::Wikidata,
    name: "PETSCII character graphics",
    extensions: &["psci"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
