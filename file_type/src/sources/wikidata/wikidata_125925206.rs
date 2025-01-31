use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125925206: FileFormat = FileFormat {
    id: 125_925_206,
    puid: "wikidata/125925206",
    name: "Papyrus Author database",
    extensions: &["pb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
