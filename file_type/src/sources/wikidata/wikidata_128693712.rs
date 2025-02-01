use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128693712: FileFormat = FileFormat {
    id: 128_693_712,
    puid: "wikidata/128693712",
    name: "Berry source code file",
    extensions: &["be", "be"],
    media_types: &["application/x-berry", "text/x-berry"],
    internal_signatures: &[],
    related_formats: &[],
};
