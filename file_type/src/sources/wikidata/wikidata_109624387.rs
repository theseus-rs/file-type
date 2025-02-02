use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109624387: FileFormat = FileFormat {
    id: 109_624_387,
    source_type: SourceType::Wikidata,
    name: "WebPlus Essentials Templates",
    extensions: &["wpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
