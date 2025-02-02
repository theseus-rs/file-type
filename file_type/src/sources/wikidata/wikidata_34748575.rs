use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34748575: FileFormat = FileFormat {
    id: 34_748_575,
    source_type: SourceType::Wikidata,
    name: "Thermo-Calc Database Format",
    extensions: &["tdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
