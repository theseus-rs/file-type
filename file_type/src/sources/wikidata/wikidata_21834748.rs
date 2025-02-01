use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21834748: FileFormat = FileFormat {
    id: 21_834_748,
    puid: "wikidata/21834748",
    name: "Adobe Color Swatch",
    extensions: &["aco"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
