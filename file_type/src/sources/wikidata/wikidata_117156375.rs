use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117156375: FileFormat = FileFormat {
    id: 117_156_375,
    puid: "wikidata/117156375",
    name: "Pyro data disc project file",
    extensions: &["cwd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
