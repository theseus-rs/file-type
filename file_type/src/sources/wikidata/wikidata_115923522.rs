use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115923522: FileFormat = FileFormat {
    id: 115_923_522,
    puid: "wikidata/115923522",
    name: "HCL configuration file",
    extensions: &["hcl", "tf"],
    media_types: &["text/x-hcl", "text/x-hcl"],
    internal_signatures: &[],
    related_formats: &[],
};
