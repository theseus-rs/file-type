use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115923522: FileFormat = FileFormat {
    id: 115_923_522,
    source_type: SourceType::Wikidata,
    name: "HCL configuration file",
    extensions: &["hcl", "tf"],
    media_types: &["text/x-hcl"],
    signatures: &[],
    related_formats: &[],
};
