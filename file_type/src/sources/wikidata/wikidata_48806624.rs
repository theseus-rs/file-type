use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48806624: FileFormat = FileFormat {
    id: 48_806_624,
    puid: "wikidata/48806624",
    name: "Corel Chart",
    extensions: &["cch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
