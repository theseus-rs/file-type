use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116790677: FileFormat = FileFormat {
    id: 116_790_677,
    puid: "wikidata/116790677",
    name: "Prepress File",
    extensions: &["sep"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
