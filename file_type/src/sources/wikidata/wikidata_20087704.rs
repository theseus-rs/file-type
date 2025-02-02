use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_20087704: FileFormat = FileFormat {
    id: 20_087_704,
    source_type: SourceType::Wikidata,
    name: "TST",
    extensions: &["tst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
