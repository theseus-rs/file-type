use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_104889134: FileFormat = FileFormat {
    id: 104_889_134,
    source_type: SourceType::Wikidata,
    name: "Propellerhead Reason Project File",
    extensions: &["reason", "rns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
