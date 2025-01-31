use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116790608: FileFormat = FileFormat {
    id: 116_790_608,
    puid: "wikidata/116790608",
    name: "InDesign template",
    extensions: &["indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
