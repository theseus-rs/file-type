use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_8041702: FileFormat = FileFormat {
    id: 8_041_702,
    source_type: SourceType::Wikidata,
    name: "eXtended Binary",
    extensions: &["xb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
