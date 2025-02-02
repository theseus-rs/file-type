use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131304008: FileFormat = FileFormat {
    id: 131_304_008,
    source_type: SourceType::Wikidata,
    name: "Riverbed Stingray Traffic Manager file format",
    extensions: &["rts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
