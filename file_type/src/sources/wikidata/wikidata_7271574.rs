use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7271574: FileFormat = FileFormat {
    id: 7_271_574,
    source_type: SourceType::Wikidata,
    name: "Quetzal file format",
    extensions: &["glksave", "sav"],
    media_types: &["application/x-glksave"],
    internal_signatures: &[],
    related_formats: &[],
};
