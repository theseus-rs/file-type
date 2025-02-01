use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205870: FileFormat = FileFormat {
    id: 28_205_870,
    puid: "wikidata/28205870",
    name: "Crack Art",
    extensions: &["ca1", "ca2", "ca3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
