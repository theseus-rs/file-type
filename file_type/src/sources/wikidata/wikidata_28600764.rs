use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600764: FileFormat = FileFormat {
    id: 28_600_764,
    puid: "wikidata/28600764",
    name: "Electronic Arts MOV",
    extensions: &["mov"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
