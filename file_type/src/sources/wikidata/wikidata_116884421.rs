use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116884421: FileFormat = FileFormat {
    id: 116_884_421,
    puid: "wikidata/116884421",
    name: "Adobe PhotoDeluxe data",
    extensions: &["pbd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
