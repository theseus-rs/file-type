use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73515813: FileFormat = FileFormat {
    id: 73_515_813,
    source_type: SourceType::Wikidata,
    name: "Palantir WinTime Plan",
    extensions: &["pln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
