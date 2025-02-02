use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111588438: FileFormat = FileFormat {
    id: 111_588_438,
    source_type: SourceType::Wikidata,
    name: "Roxio Label Creator Project File, versions 6-7",
    extensions: &["jwl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
