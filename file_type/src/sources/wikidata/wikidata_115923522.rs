use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115923522: FileFormat = FileFormat {
    id: 115_923_522,
    source_type: SourceType::Wikidata,
    name: "HCL configuration file",
    extensions: &["hcl", "tf"],
    media_types: &["text/x-hcl"],
    internal_signatures: &[],
    related_formats: &[],
};
