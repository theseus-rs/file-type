use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96081191: FileFormat = FileFormat {
    id: 96_081_191,
    puid: "wikidata/96081191",
    name: "SystemModeler experiment format",
    extensions: &["sme"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
