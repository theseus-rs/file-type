use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975631: FileFormat = FileFormat {
    id: 28_975_631,
    source_type: SourceType::Wikidata,
    name: "Moray User Defined Object",
    extensions: &["udo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
