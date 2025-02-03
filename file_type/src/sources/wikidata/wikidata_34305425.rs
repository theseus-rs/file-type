use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34305425: FileFormat = FileFormat {
    id: 34_305_425,
    source_type: SourceType::Wikidata,
    name: "Scheme script",
    extensions: &["sch", "scm", "ss"],
    media_types: &["application/x-scheme", "text/x-scheme"],
    internal_signatures: &[],
    related_formats: &[],
};
