use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47012486: FileFormat = FileFormat {
    id: 47_012_486,
    puid: "wikidata/47012486",
    name: "MultiMate Text File",
    extensions: &["dox", "fnx", "pat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
